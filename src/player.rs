pub mod player {
    use macroquad::prelude::*;
    use macroquad::time::get_frame_time;
    use crate::display::display::Settings;
    use crate::map::world::World;
    use crate::raycast::raycast::Ray;
    use core::f32::consts::PI;

    /* 
        Player settings, input and movement
    */
    pub struct Background {
        pub img: Image,
        pub texture: Texture2D,
    }
    pub struct Player {
        pub pos: Vec3, // Start vector
        pub dir: Vec2, // Inital direction vector
        pub plane: Vec2, // Camera plane
        pub world: World, 
        pub ds: Settings, 
        pub pitch: f32,
        pub timer: f32,
        background: Background, 
        zbuffer: Vec<f32>, 
    }
    
    impl Player {
        pub fn new(world: World, display_settings: Settings) -> Player {
            // Background image used to draw the floors
            let img = Image::gen_image_color(display_settings.width as u16, display_settings.height as u16, BLACK);
            let background: Background = Background { 
                texture: Texture2D::from_image(&img),
                img: img,
            };

            let zbuffer = vec![0.0; display_settings.width as usize];

            Player { 
                pos: vec3(22.0, 11.5, 0.0),
                dir: vec2(-1.0, 0.0),
                plane: vec2(0.0, 0.66),
                world: world,  
                ds: display_settings,  
                pitch: 0.0,
                timer: 0.0,
                zbuffer: zbuffer,
                background: background,
            }
        }

        pub fn draw_floor(&mut self){      
            let t_height = self.world.textures[self.ds.floor_texture].texture.height();

            for y in 0..self.ds.height as i32{
                let is_floor = y > (self.ds.half_height + self.pitch) as i32;
                let ray_dir_0: Vec2 = vec2(self.dir.x - self.plane.x, self.dir.y - self.plane.y);
                let ray_dir_1: Vec2 = vec2(self.dir.x + self.plane.x, self.dir.y + self.plane.y);

                let p = if is_floor 
                        { y - self.ds.half_height as i32 - self.pitch as i32 }
                else    { self.ds.half_height as i32 - y + self.pitch as i32 };

                let cam_z = if is_floor 
                        { 0.5 * self.ds.height + self.pos.z }
                else    { 0.5 * self.ds.height - self.pos.z };

                let row_distance = cam_z / p as f32;
                let floor_step: Vec2 = vec2(
                    row_distance * (ray_dir_1.x - ray_dir_0.x) / self.ds.width, 
                    row_distance * (ray_dir_1.y - ray_dir_0.y) / self.ds.width
                );

                let mut floor: Vec2 = vec2(self.pos.x + row_distance * ray_dir_0.x, self.pos.y + row_distance * ray_dir_0.y);
                
                for x in 0..self.ds.width as u32 {
                    let tx = (t_height * floor.x) as i32 & (t_height - 1.0) as i32;
                    let ty = (t_height * floor.y) as i32 & (t_height - 1.0) as i32;
                    floor.x += floor_step.x; floor.y += floor_step.y;

                    if is_floor {
                        let mut floor_p: Color = self.world.textures[self.ds.floor_texture].texture_data[(t_height as i32 * tx + ty) as usize];  
                        if !self.ds.nightvision { 
                            floor_p = self.world.floor_shading(floor_p, 
                                (self.ds.height + 50.0) as i32, 
                                y, 
                                self.ds.floor_shading_multiplier, 
                                self.ds.dark_shading 
                            ); 
                        }
                        self.background.img.set_pixel(x as u32, y as u32, floor_p); 
                    }
                    else {
                        let mut ceil_p: Color = self.world.textures[self.ds.ceil_texture].texture_data[(t_height as i32 * tx + ty) as usize];  
                        if !self.ds.nightvision { 
                            ceil_p = self.world.floor_shading(ceil_p, 
                                (self.ds.height + 30.0) as i32,
                                y, 
                                self.ds.ceil_shading_multiplier,
                                self.ds.dark_shading
                            ); 
                        }
                        self.background.img.set_pixel(x as u32, y as u32, ceil_p); 
                    }
                }
            }
            let c: Color = if self.ds.nightvision { GREEN } else { WHITE };            
            self.background.texture.update(&self.background.img); 
            draw_texture(self.background.texture, 0., 0., c);
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
            let draw_start: f32 = -line_height / 2.0 + self.pitch + self.ds.half_height + (self.pos.z / ray.perp_wall_dist);
            self.zbuffer[x as usize] = ray.perp_wall_dist; // Store dist of wall strip in buffer for spritecast

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
            let num_sprites = self.world.sprite_map.len();
            
            // Calculate dist to sprite
            for i in 0..num_sprites {
                self.world.sprite_map[i].dist = 
                    (self.pos.x - self.world.sprite_map[i].x) 
                    * (self.pos.x - self.world.sprite_map[i].x) 
                    + (self.pos.y - self.world.sprite_map[i].y) 
                    * (self.pos.y - self.world.sprite_map[i].y)
            }

            // Sort sprites
            self.world.sprite_map.sort_by(|a, b| b.dist.partial_cmp(&a.dist).unwrap());

            // Draw sprites
            for i in 0..num_sprites {
                let c: Color = if self.ds.nightvision { GREEN } 
                else { 
                    self.world.sprite_shading(
                        self.world.sprite_map[i].dist, 
                        self.ds.dark_shading, 
                        self.ds.sprite_shading_multiplier
                    ) 
                };

                let sprite: Vec2 = vec2(self.world.sprite_map[i].x - self.pos.x, self.world.sprite_map[i].y - self.pos.y);
                let inv_det: f32 = 1.0 / (self.plane.x * self.dir.y - self.dir.x * self.plane.y);
                let transform: Vec2 = vec2(
                    inv_det * (self.dir.y * sprite.x - self.dir.x * sprite.y), 
                    inv_det * (-self.plane.y * sprite.x + self.plane.x * sprite.y)
                );

                let v_move_screen = (self.pitch + self.pos.z / transform.y) as i32;
                let sprite_screen: i32 = (self.ds.half_width * (1.0 + transform.x / transform.y)) as i32;
                let sprite_height: i32 = (self.ds.height / transform.y) as i32;
                let half_sprite_height: (i32, i32) = (-sprite_height / 2, sprite_height / 2);

                let draw_start_y: i32 = half_sprite_height.0 + self.ds.half_height as i32 + v_move_screen;
                let draw_end_y: i32 = half_sprite_height.1 + self.ds.half_height as i32 + v_move_screen;
                let draw_start_x: i32 = half_sprite_height.0 + sprite_screen;
                let draw_end_x: i32 = half_sprite_height.1 + sprite_screen;

                for x in draw_start_x..draw_end_x {
                    let t = self.world.sprite_map[i].texture;
                    let tex_x: i32 = ((x - draw_start_x) * t.height() as i32 / sprite_height) as i32;
                    if transform.y > 0.0 && x > 0 && x < self.ds.width as i32 && transform.y < self.zbuffer[x as usize] {

                        draw_texture_ex(
                            t,
                            x as f32,
                            draw_start_y as f32,
                            c,
                            DrawTextureParams {
                                dest_size: Some(vec2(1.0, (draw_end_y - draw_start_y) as f32)), 
                                source: Some(Rect::new(tex_x as f32, 0.0, 1.0, t.height())), // Part of texture to draw
                                ..Default::default()
                            }
                        );
                    }
                }
            }
        }

        pub fn raycast(&mut self) { 
            for x in 0..self.ds.width as u32 {
                let mut ray:Ray = Ray::new(x as f32, self);
                ray.dda(self);
                self.draw_walls(ray, x as f32);
            }
        }

        pub fn headbob(&mut self) { 
            self.timer += get_frame_time() * self.ds.headbob_speed;
            self.pos.z += self.timer.sin() * self.ds.headbob_amount;
            if self.timer > PI * 2.0 { self.timer = 0.0; }
        }

        pub fn movement(&mut self) {
            if is_key_down(KeyCode::W) {
                self.move_forward();
                if self.ds.headbob { self.headbob(); }
            }
    
            if is_key_down(KeyCode::S) {
                self.move_down();
                if self.ds.headbob { self.headbob(); }
            }
    
            if is_key_down(KeyCode::D) {
                self.move_right();
            }
    
            if is_key_down(KeyCode::A) {
                self.move_left();
            }

            if is_key_down(KeyCode::Q) {
                self.pitch += 3.0 * get_frame_time() * self.ds.look_speed;
                if self.pitch > 500.0 { self.pitch = 500.0 }
            }

            if is_key_down(KeyCode::E) {
                self.pitch -= 3.0 * get_frame_time() * self.ds.look_speed;
                if self.pitch < -500.0 { self.pitch = -500.0 }
            }

            if is_key_down(KeyCode::Z) {
                self.pos.z += 3.0 * get_frame_time() * self.ds.look_speed;
            }

            if is_key_down(KeyCode::X) {
                self.pos.z -= 3.0 * get_frame_time() * self.ds.look_speed;
            }

            if is_key_pressed(KeyCode::Tab) {
                self.ds.settings = !self.ds.settings;
            }

            if self.pos.z > 200.0 { self.pos.z = 200.0 }
            if self.pos.z < -200.0 { self.pos.z = -200.0 }
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

