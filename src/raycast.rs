pub mod raycast {
    use macroquad::prelude::*;
    use crate::player::player::Player;

    /* Raycast logic */

    pub struct Ray {
        pub camera: f32,  
        pub ray_dir: Vec2,  
        pub map: (i32, i32),  
        pub side_dist: Vec2,  
        pub delta_dist: Vec2,  
        pub perp_wall_dist: f32,  
        pub step: (i32, i32),
        pub hit: bool, 
        pub side: bool,
    }

    impl Ray {
        pub fn new(x: f32, player: &mut Player) -> Ray {     
            let camera: f32 = 2.0 * x / player.width - 1.0; 
            let ray_dir: Vec2 = vec2(player.dir.x + player.plane.x * camera, player.dir.y + player.plane.y * camera);
            let map: (i32, i32) = (player.pos.x as i32, player.pos.y as i32); 
            let mut side_dist: Vec2 = vec2(0.0, 0.0);
            let delta_dist: Vec2 = vec2((1.0 / ray_dir.x).abs(), (1.0 / ray_dir.y).abs());  
            let perp_wall_dist: f32 = 0.0; 
            let mut step: (i32, i32) = (0, 0); 

            if ray_dir.x < 0.0 { step.0 = -1; side_dist.x = (player.pos.x - map.0 as f32) * delta_dist.x; }
            else { step.0 = 1; side_dist.x = (map.0 as f32 + 1.0 - player.pos.x) * delta_dist.x; } 
            if ray_dir.y < 0.0 { step.1 = -1; side_dist.y = (player.pos.y - map.1 as f32) * delta_dist.y; }
            else { step.1 = 1; side_dist.y = (map.1 as f32 + 1.0 - player.pos.y) * delta_dist.y; }

            Ray { 
                camera: camera,
                ray_dir: ray_dir,
                map: map,
                side_dist: side_dist,
                delta_dist: delta_dist,
                perp_wall_dist: perp_wall_dist,
                step: step,
                hit: false,
                side: false,
            }
        }

        pub fn dda(&mut self, player: &mut Player) {
            while !self.hit {
                if self.side_dist.x < self.side_dist.y {
                    self.side_dist.x += self.delta_dist.x;
                    self.map.0 += self.step.0;
                    self.side = false;
                }

                else {
                    self.side_dist.y += self.delta_dist.y;
                    self.map.1 += self.step.1;
                    self.side = true;
                }
                // If ray hit wall
                if player.world.get(self.map.0, self.map.1) > 0 { self.hit = true; }
            }
            // Calculate distance projected on camera direction
            if !self.side   { self.perp_wall_dist = self.side_dist.x - self.delta_dist.x; }
            else            { self.perp_wall_dist = self.side_dist.y - self.delta_dist.y; }
        }
    }
}

