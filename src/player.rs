pub mod player {
    use macroquad::prelude::*;
    use macroquad::time::get_frame_time;
    use crate::display::display::Settings;
    use crate::map::world::World;
    use crate::raycast::raycast::Ray;

    /* 
        Player settings, input and movement
    */
    pub struct Background {
        pub img: Image,
        pub texture: Texture2D,
    }
    pub struct Player {
        pub pos: Vec2, // Start vector
        pub dir: Vec2, // Inital direction vector
        pub plane: Vec2, // Camera plane
        pub world: World, 
        pub ds: Settings, 
        background: Background, 
    }
    
    impl Player {
        pub fn new(world: World, display_settings: Settings) -> Player {
            // Background image used to draw the floors
            let img = Image::gen_image_color(display_settings.width as u16, display_settings.height as u16, BLACK);
            let background: Background = Background { 
                texture: Texture2D::from_image(&img),
                img: img,
            };

            Player { 
                pos: vec2(22.0, 11.5),
                dir: vec2(-1.0, 0.0),
                plane: vec2(0.0, 0.66),
                world: world,  
                ds: display_settings,  
                background: background,
            }
        }

        pub fn draw_floor(&mut self){      
            let mut c: Color = WHITE;
            if self.ds.fast_floors {
                if self.ds.dark_shading {
                    for y in self.ds.half as i32 + 1..self.ds.height as i32 {
                        c = if self.ds.nightvision { GREEN } else { GRAY };
                        c = self.world.floor_shading(c, (self.ds.height + 50.0) as i32, y, 0.09, self.ds.dark_shading); 
                        draw_line(0.0, y as f32, self.ds.width, y as f32, 1.0, c);

                        c = if self.ds.nightvision { DARKGREEN } else { DARKGRAY };
                        c = self.world.floor_shading(c, (self.ds.height + 30.0) as i32, y, 0.1, self.ds.dark_shading); 
                        draw_line(0.0, (self.ds.height as i32 - y) as f32, self.ds.width, (self.ds.height as i32 - y) as f32, 1.0, c);
                    }
                }
                else {
                    draw_rectangle(0.0, 0.0, self.ds.width, self.ds.half , DARKGRAY);
                    draw_rectangle(0.0, self.ds.half, self.ds.width, self.ds.half, GRAY);
                }
            }
            else {
                let floor_texture_height = self.world.textures[self.ds.floor_texture].texture.height();

                for y in self.ds.half as i32 + 1..self.ds.height as i32 {
                    let ray_dir_0: Vec2 = vec2(self.dir.x - self.plane.x, self.dir.y - self.plane.y);
                    let ray_dir_1: Vec2 = vec2(self.dir.x + self.plane.x, self.dir.y + self.plane.y);
                    let row_distance = (0.5 * self.ds.height) / (y - self.ds.half as i32) as f32;

                    let floor_step: Vec2 = vec2(
                        row_distance * (ray_dir_1.x - ray_dir_0.x) / self.ds.width, 
                        row_distance * (ray_dir_1.y - ray_dir_0.y) / self.ds.width
                    );
                    let mut floor: Vec2 = vec2(self.pos.x + row_distance * ray_dir_0.x, self.pos.y + row_distance * ray_dir_0.y);
                
                    for x in 0..self.ds.width as u32 {
                        let tx = (floor_texture_height * floor.x) as i32 & (floor_texture_height - 1.0) as i32;
                        let ty = (floor_texture_height * floor.y) as i32 & (floor_texture_height - 1.0) as i32;
                        floor.x += floor_step.x; floor.y += floor_step.y;

                        let mut floor_p: Color = self.world.textures[self.ds.floor_texture].texture_data[(floor_texture_height as i32 * tx + ty) as usize];  
                        if !self.ds.nightvision { 
                            floor_p = self.world.floor_shading(floor_p, 
                                (self.ds.height + 50.0) as i32, 
                                y, 
                                self.ds.floor_shading_multiplier, 
                                self.ds.dark_shading 
                            ); 
                        }
                        self.background.img.set_pixel(x as u32, y as u32, floor_p); 

                        let mut ceil_p: Color = self.world.textures[self.ds.ceil_texture].texture_data[(floor_texture_height as i32 * tx + ty) as usize];  
                        if !self.ds.nightvision { 
                            ceil_p = self.world.floor_shading(ceil_p, 
                                (self.ds.height + 30.0) as i32,
                                y, 
                                self.ds.ceil_shading_multiplier,
                                self.ds.dark_shading
                            ); 
                        }
                        self.background.img.set_pixel(x as u32, (self.ds.height as i32 - y - 1) as u32, ceil_p); 
                    }
                }
                if self.ds.nightvision { c = GREEN };
                self.background.texture.update(&self.background.img); 
                draw_texture(self.background.texture, 0., 0., c);
            }
        }

        pub fn draw_walls(&mut self, ray: Ray, x: f32) {
            let c: Color = if self.ds.nightvision { GREEN } 
            else { 
                self.world.wall_shading(
                    ray.side, 
                    ray.perp_wall_dist, 
                    self.ds.shadows, 
                    self.ds.dark_shading, 
                    self.ds.wall_shading_multiplier
                ) 
            };
            let t: Texture2D = self.world.texture(ray.map);
        
            let line_height: f32 = self.ds.height / ray.perp_wall_dist;
            let draw_start: f32 = -line_height / 2.0 + self.ds.height / 2.0;

            let mut wall_x: f32;
            if !ray.side { wall_x = self.pos.y + ray.perp_wall_dist * ray.ray_dir.y; }
            else         { wall_x = self.pos.x + ray.perp_wall_dist * ray.ray_dir.x; }
            wall_x -= wall_x.floor();

            let mut tex_x: u32 = (wall_x * t.height()) as u32;
            if !ray.side && ray.ray_dir.x > 0.0 { tex_x = t.height() as u32 - tex_x - 1}
            if  ray.side && ray.ray_dir.y < 0.0 { tex_x = t.height() as u32 - tex_x - 1}  

            draw_texture_ex(
                t,
                x as f32,
                draw_start as f32,
                c,
                DrawTextureParams {
                    dest_size: Some(vec2(1.0, line_height as f32)), 
                    source: Some(Rect::new(tex_x as f32, 0.0, 1.0, t.height())), // Part of texture to draw
                    ..Default::default()
                }
            );
        }

        pub fn draw_sprites(&mut self) {
            // todo
        }

        pub fn raycast(&mut self) {  // Add more variables 
            for x in 0..self.ds.width as u32 {
                let mut ray:Ray = Ray::new(x as f32, self);
                ray.dda(self);
                self.draw_walls(ray, x as f32);
            }
        }

        pub fn movement(&mut self) {
            if is_key_down(KeyCode::W) {
                self.move_forward();
            }
    
            if is_key_down(KeyCode::S) {
                self.move_down();
            }
    
            if is_key_down(KeyCode::D) {
                self.move_right();
            }
    
            if is_key_down(KeyCode::A) {
                self.move_left();
            }
    
            if is_key_pressed(KeyCode::Tab) {
                self.ds.settings = !self.ds.settings;
            }
        }

        fn move_forward(&mut self) {
            let m: f32 = get_frame_time() * self.ds.move_speed;
            if self.world.get((self.pos.x + self.dir.x * m) as i32, self.pos.y as i32) == 0 { 
                self.pos.x += self.dir.x * m; 
            }
            if self.world.get(self.pos.x as i32, (self.pos.y + self.dir.y * m) as i32) == 0 { 
                self.pos.y += self.dir.y * m; 
            }
        }
    
        fn move_down(&mut self) {
            let m: f32 = get_frame_time() * self.ds.move_speed;
            if self.world.get((self.pos.x - self.dir.x * m) as i32, self.pos.y as i32) == 0 { 
                self.pos.x -= self.dir.x * m; 
            }
            if self.world.get(self.pos.x as i32, (self.pos.y - self.dir.y * m) as i32) == 0 { 
                self.pos.y -= self.dir.y * m; 
            }
        }
    
        fn move_right(&mut self) {
            let r: f32 = get_frame_time() * self.ds.rot_speed;
            let old_dir_x: f32 = self.dir.x;
            self.dir.x = self.dir.x * (-r).cos() - self.dir.y * (-r).sin();
            self.dir.y = old_dir_x * (-r).sin() + self.dir.y * (-r).cos();
            let old_plane_x: f32 = self.plane.x;
            self.plane.x = self.plane.x * (-r).cos() - self.plane.y * (-r).sin();
            self.plane.y = old_plane_x * (-r).sin() + self.plane.y * (-r).cos();
        }
    
        fn move_left(&mut self) {
            let r: f32 = get_frame_time() * self.ds.rot_speed;
            let old_dir_x: f32 = self.dir.x;
            self.dir.x = self.dir.x * r.cos() - self.dir.y * r.sin();
            self.dir.y = old_dir_x * r.sin() + self.dir.y * r.cos();
            let old_plane_x: f32 = self.plane.x;
            self.plane.x = self.plane.x * r.cos() - self.plane.y * r.sin();
            self.plane.y = old_plane_x * r.sin() + self.plane.y * r.cos();
        }
    }
}

