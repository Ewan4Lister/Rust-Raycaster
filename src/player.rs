pub mod player {
    use macroquad::prelude::*;
    use macroquad::time::get_frame_time;
    use crate::display::display::Display;
    use crate::map::world::World;
    use crate::raycast::raycast::Ray;

    pub struct Background {
        pub img: Image,
        pub texture: Texture2D,
        pub fast_floors: bool,
    }
    pub struct Player {
        pub pos: Vec2, // start vector
        pub dir: Vec2, // inital direction vector
        pub plane: Vec2, // camera plane
        pub world: World, 
        pub display: Display, 
        pub width: f32,
        pub height: f32,
        pub half: f32,
        background: Background,
        move_speed: f32,
        rot_speed: f32,
        pub nightvision: bool,        
    }
    
    impl Player {
        pub fn new(world: World, display: Display) -> Player {
            let img = Image::gen_image_color(screen_width() as u16, screen_height() as u16, BLACK);
            let background: Background = Background { 
                texture: Texture2D::from_image(&img),
                img: img,
                fast_floors: true,
            };

            Player { 
                move_speed: 4.0,    
                rot_speed: 3.0,   
                pos: vec2(22.0, 11.5),
                dir: vec2(-1.0, 0.0),
                plane: vec2(0.0, 0.66),
                width: screen_width(),
                height: screen_height(),
                half: screen_height() / 2.0,
                background: background,
                world: world,  
                display: display,  
                nightvision: false,
            }
        }

        pub fn draw_floor(&mut self){      
            let mut c: Color = WHITE;
            if self.background.fast_floors {
                if self.world.dark_shading {
                    for y in self.half as i32 + 1..self.height as i32 {
                        c = if self.nightvision { GREEN } else { GRAY };
                        c = self.world.floor_shading(c, (self.height + 50.0) as i32, y, 0.09); 
                        draw_line(0.0, y as f32, self.width, y as f32, 1.0, c);

                        c = if self.nightvision { DARKGREEN } else { DARKGRAY };
                        c = self.world.floor_shading(c, (self.height + 30.0) as i32, y, 0.1); 
                        draw_line(0.0, (self.height as i32 - y) as f32, self.width, (self.height as i32 - y) as f32, 1.0, c);
                    }
                }
                else {
                    draw_rectangle(0.0, 0.0, self.width, self.half , DARKGRAY);
                    draw_rectangle(0.0, self.half, self.width, self.half, GRAY);
                }
            }
            else {
                let threshold = 10;
                let floor_texture = 3;
                let ceil_texture = 6;
                let floor_texture_height = self.world.textures[floor_texture].texture.height();

                for y in threshold + self.half as i32 + 1..self.height as i32 {
                    let ray_dir_0: Vec2 = vec2(self.dir.x - self.plane.x, self.dir.y - self.plane.y);
                    let ray_dir_1: Vec2 = vec2(self.dir.x + self.plane.x, self.dir.y + self.plane.y);
                    let row_distance = (0.5 * self.height) / (y - self.half as i32) as f32;

                    let floor_step: Vec2 = vec2(
                        row_distance * (ray_dir_1.x - ray_dir_0.x) / self.width, 
                        row_distance * (ray_dir_1.y - ray_dir_0.y) / self.width
                    );
                    let mut floor: Vec2 = vec2(self.pos.x + row_distance * ray_dir_0.x, self.pos.y + row_distance * ray_dir_0.y);
                
                    for x in 0..self.width as u32 {
                        let cell = (floor.x as i32, floor.y as i32);
                        let tx = (floor_texture_height * (floor.x - cell.0 as f32)) as i32 & (floor_texture_height - 1.0) as i32;
                        let ty = (floor_texture_height * (floor.y - cell.1 as f32)) as i32 & (floor_texture_height - 1.0) as i32;
                        floor.x += floor_step.x; floor.y += floor_step.y;

                        let mut floor_p: Color = self.world.textures[floor_texture].texture_data[(floor_texture_height as i32 * tx + ty) as usize];  
                        if !self.nightvision { floor_p = self.world.floor_shading(floor_p, (self.height + 50.0) as i32, y, 0.08); }
                        self.background.img.set_pixel(x as u32, y as u32, floor_p); 

                        let mut ceil_p: Color = self.world.textures[ceil_texture].texture_data[(floor_texture_height as i32 * tx + ty) as usize];  
                        if !self.nightvision { ceil_p = self.world.floor_shading(ceil_p, (self.height + 30.0) as i32, y, 0.1); }
                        self.background.img.set_pixel(x as u32, (self.height as i32 - y - 1) as u32, ceil_p); 
                    }
                }
                if self.nightvision { c = GREEN };
                self.background.texture.update(&self.background.img); 
                draw_texture(self.background.texture, 0., 0., c);
            }
        }

        pub fn draw_walls(&mut self, ray: Ray, x: f32) {
            let c: Color = if self.nightvision { GREEN } else { self.world.wall_shading(ray.side, ray.perp_wall_dist) };
            let t: Texture2D = self.world.texture(ray.map);
        
            let line_height: f32 = self.height / ray.perp_wall_dist;
            let  draw_start: f32 = -line_height / 2.0 + self.height / 2.0;

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

        }

        pub fn raycast(&mut self) {  // Add more variables 
            for x in 0..self.width as u32 {
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
    
            if is_key_pressed(KeyCode::F1) {
                self.world.shadows = !self.world.shadows;
            }

            if is_key_pressed(KeyCode::F2) {
                self.world.dark_shading = !self.world.dark_shading;
            }
            
            if is_key_pressed(KeyCode::F3) {
                self.nightvision = !self.nightvision;
            }

            if is_key_pressed(KeyCode::F4) {
                self.background.fast_floors = !self.background.fast_floors;
            }
        }

        fn move_forward(&mut self) {
            let m: f32 = get_frame_time() * self.move_speed;
            if self.world.get((self.pos.x + self.dir.x * m) as i32, self.pos.y as i32) == 0 { 
                self.pos.x += self.dir.x * m; 
            }
            if self.world.get(self.pos.x as i32, (self.pos.y + self.dir.y * m) as i32) == 0 { 
                self.pos.y += self.dir.y * m; 
            }
        }
    
        fn move_down(&mut self) {
            let m: f32 = get_frame_time() * self.move_speed;
            if self.world.get((self.pos.x - self.dir.x * m) as i32, self.pos.y as i32) == 0 { 
                self.pos.x -= self.dir.x * m; 
            }
            if self.world.get(self.pos.x as i32, (self.pos.y - self.dir.y * m) as i32) == 0 { 
                self.pos.y -= self.dir.y * m; 
            }
        }
    
        fn move_right(&mut self) {
            let r: f32 = get_frame_time() * self.rot_speed;
            let old_dir_x: f32 = self.dir.x;
            self.dir.x = self.dir.x * (-r).cos() - self.dir.y * (-r).sin();
            self.dir.y = old_dir_x * (-r).sin() + self.dir.y * (-r).cos();
            let old_plane_x: f32 = self.plane.x;
            self.plane.x = self.plane.x * (-r).cos() - self.plane.y * (-r).sin();
            self.plane.y = old_plane_x * (-r).sin() + self.plane.y * (-r).cos();
        }
    
        fn move_left(&mut self) {
            let r: f32 = get_frame_time() * self.rot_speed;
            let old_dir_x: f32 = self.dir.x;
            self.dir.x = self.dir.x * r.cos() - self.dir.y * r.sin();
            self.dir.y = old_dir_x * r.sin() + self.dir.y * r.cos();
            let old_plane_x: f32 = self.plane.x;
            self.plane.x = self.plane.x * r.cos() - self.plane.y * r.sin();
            self.plane.y = old_plane_x * r.sin() + self.plane.y * r.cos();
        }
    }
}

