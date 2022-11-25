pub mod world {
    use macroquad::prelude::*;

    pub struct Sprite {
        pub x: f32,
        pub y: f32,
        pub dist: f32,
        pub texture: Texture2D,
    }
    pub struct Texture {
        pub texture: Texture2D,
        pub texture_data: Vec<Color>,
    }
    pub enum Entity { 
        Door((u32, u32), (i32, i32)), 
        Power((u32, u32), (i32, i32))
     }

    async fn load_textures(texture_strings: Vec<&str>) -> Vec<Texture> {
        let mut textures: Vec<Texture> = Vec::new();
        for n in texture_strings {
            let texture: Texture2D = load_texture(n).await.unwrap();
            let mut texture_data: Vec<Color> = Vec::new();
            for p in load_image(n).await.unwrap().get_image_data().to_vec() {
                let c: Color = p.into();
                texture_data.push(c);
            }
            textures.push(Texture { texture: texture, texture_data: texture_data });
        }
        textures
    }
    pub struct World {
        pub world_map: Vec<u32>,
        pub textures: Vec<Texture>,
        pub sprite_map: Vec<Sprite>,
        pub entities: Vec<Entity>,
        pub columns: usize,
    }

    impl World {
        pub async fn new() -> World {

            let world_map: Vec<u32> = vec![
                1,1,1,1,1,1,1,1,1,1,1,3,2,3,2,3,2,3,2,3,2,3,2,3,
                1,0,0,0,0,0,0,0,0,0,1,2,0,0,0,0,0,0,0,0,0,0,0,2,
                1,0,5,5,0,5,5,5,0,1,1,3,0,0,0,0,0,0,0,0,0,0,0,3,
                1,0,0,0,0,0,0,0,0,0,12,0,0,0,0,0,0,0,0,0,0,0,0,2,
                1,0,5,5,0,5,5,5,0,1,1,2,0,0,0,0,0,0,0,0,0,0,0,3,
                1,0,0,0,0,0,0,0,0,0,1,3,0,0,0,0,0,6,6,6,0,6,6,6,
                1,1,1,1,0,1,1,1,1,1,1,2,3,2,3,2,3,6,0,6,0,6,0,6,
                5,5,5,5,0,5,5,5,5,0,8,0,8,0,8,0,7,6,0,0,0,0,0,6,
                5,5,0,0,0,0,0,0,5,7,0,7,0,7,0,7,7,6,0,0,0,0,0,6,
                5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,6,0,0,0,0,0,6,
                5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,6,0,6,0,6,0,6,
                5,5,0,0,0,0,0,0,5,7,0,7,0,7,0,7,7,6,6,8,0,8,6,6,
                5,5,5,5,0,5,5,5,5,7,8,7,8,7,8,7,7,8,8,0,0,0,8,8,
                2,2,2,2,0,2,2,2,2,7,7,7,0,7,7,0,7,8,0,0,0,0,0,8,
                2,2,0,0,0,0,0,2,2,7,0,0,0,0,0,0,7,8,0,0,0,0,0,8,
                2,0,0,0,0,0,0,0,2,7,0,0,0,0,0,0,7,8,0,0,0,0,0,8,
                2,0,0,0,0,0,0,0,2,7,7,7,7,7,7,0,7,8,8,0,0,0,8,8,
                2,0,0,0,0,0,0,0,2,1,1,1,1,1,2,13,7,0,0,8,0,8,0,4,
                9,9,9,9,11,9,9,9,9,1,0,0,0,1,1,0,4,0,4,0,0,0,4,4,
                9,0,0,0,0,0,0,0,1,0,0,0,0,0,1,4,0,4,0,4,0,4,0,4,
                9,0,0,0,0,0,0,0,12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,
                9,0,0,0,0,0,0,0,1,0,0,0,0,0,1,4,0,4,0,4,0,4,0,4,
                9,9,0,0,0,0,0,9,1,1,0,0,0,1,1,0,4,0,4,0,0,0,4,4,
                9,9,9,9,9,9,9,9,2,1,1,1,1,1,1,4,4,4,4,4,4,4,4,4
            ];

            let entities = vec![
                Entity::Door((0, 12), (20, 8)),
                Entity::Door((0, 11), (18, 4)),
                Entity::Door((0, 8), (12, 12)),
                Entity::Power((14, 13), (17, 15)),
                Entity::Door((0, 12), (3, 10)),
            ];

            let sprites: Vec<Texture2D> = vec![
                load_texture("src/assets/barrel.png").await.unwrap(), 
                load_texture("src/assets/pillar.png").await.unwrap(), 
                load_texture("src/assets/greenlight.png").await.unwrap(), 
                load_texture("src/assets/jerma.png").await.unwrap(), 
            ];

            let sprite_map: Vec<Sprite> = vec![
                Sprite {x: 13.5,    y: 15.5,    texture: sprites[3],     dist: 0.0},
                Sprite {x: 20.5,    y: 11.5,    texture: sprites[2],     dist: 0.0},
                Sprite {x: 18.5,    y: 4.5,     texture: sprites[2],     dist: 0.0},
                Sprite {x: 10.0,    y: 4.5,     texture: sprites[2],     dist: 0.0},
                Sprite {x: 10.0,    y: 12.5,    texture: sprites[2],     dist: 0.0},
                Sprite {x: 3.5,     y: 6.5,     texture: sprites[2],     dist: 0.0},
                Sprite {x: 3.5,     y: 20.5,    texture: sprites[2],     dist: 0.0},
                Sprite {x: 3.5,     y: 14.5,    texture: sprites[2],     dist: 0.0},
                Sprite {x: 14.5,    y: 20.5,    texture: sprites[2],     dist: 0.0},
                Sprite {x: 18.5,    y: 10.5,    texture: sprites[1],     dist: 0.0},
                Sprite {x: 18.5,    y: 11.5,    texture: sprites[1],     dist: 0.0},
                Sprite {x: 18.5,    y: 12.5,    texture: sprites[1],     dist: 0.0},
                Sprite {x: 21.5,    y: 1.5,     texture: sprites[0],     dist: 0.0},
                Sprite {x: 15.5,    y: 1.5,     texture: sprites[0],     dist: 0.0},
                Sprite {x: 16.0,    y: 1.8,     texture: sprites[0],     dist: 0.0},
                Sprite {x: 16.2,    y: 1.2,     texture: sprites[0],     dist: 0.0},
                Sprite {x: 3.5,     y: 2.5,     texture: sprites[0],     dist: 0.0},
                Sprite {x: 9.5,     y: 15.5,    texture: sprites[0],     dist: 0.0},
                Sprite {x: 10.0,    y: 15.1,    texture: sprites[0],     dist: 0.0},
                Sprite {x: 10.5,    y: 15.8,    texture: sprites[0],     dist: 0.0},
            ];

            let textures_names: Vec<&str> = vec![
                "src/assets/red_brick.png",         // 1
                "src/assets/concrete_pattern.png",  // 2
                "src/assets/smooth_concrete.png",   // 3
                "src/assets/mossy_cobble.png",      // 4
                "src/assets/metal_floor.png",       // 5
                "src/assets/rose_dark.png",         // 6
                "src/assets/rose_pattern.png",      // 7
                "src/assets/floral_pattern.png",    // 8
                "src/assets/wood_plank.png",        // 9
                "src/assets/log_plank.png",         // 10
                "src/assets/wooden_double_door.png",// 11
                "src/assets/metal_double_door.png", // 12
                "src/assets/button_off.png",        // 13
                "src/assets/button_on.png",         // 14
            ];

            let textures: Vec<Texture> = load_textures(textures_names).await;

            World { 
                world_map: world_map, 
                columns: 24,
                textures: textures,
                entities: entities,
                sprite_map: sprite_map, 
            }
        }
        
        pub fn get(&self, r: i32, c: i32) -> u32 {
            self.world_map[self.columns * r as usize + c as usize]
        }

        pub fn change(&mut self, pos: Vec3,  texture: (u32, u32), coords: (i32, i32)) { // Change texture in map
            if self.get(coords.0, coords.1) != texture.0 { 
                self.world_map[self.columns * coords.0 as usize + coords.1 as usize] = texture.0;
            }
            else if coords != (pos.x as i32, pos.y as i32) { 
                self.world_map[self.columns * coords.0 as usize + coords.1 as usize] = texture.1;
            }
        }

        pub fn texture(&self, map: (i32, i32)) -> Texture2D {
            self.textures[(self.get(map.0, map.1) - 1) as usize].texture
        }

        // Maybe combine these shading functions ?
        pub fn floor_shading(&self, mut color: Color, height: i32, dist: i32, multiplier: f32, dark_shading: bool) -> Color {
            if dark_shading { 
                color.r /= (height - dist) as f32 * multiplier;
                color.g /= (height - dist) as f32 * multiplier;
                color.b /= (height - dist) as f32 * multiplier;
            }

            color
        }

        pub fn wall_shading(&self, side: bool, dist: f32, shadows: bool, dark_shading: bool, shading_multiplier: f32) -> Color {
            let mut color: Color = WHITE;

            if dark_shading { 
                color.r /= dist * shading_multiplier; 
                color.g /= dist * shading_multiplier; 
                color.b /= dist * shading_multiplier;
            }
            
            if shadows { 
                if side { color.r /= 1.5; color.g /= 1.5; color.b /= 1.5; } 
            }

            color
        }

        pub fn sprite_shading(&self, dist: f32, dark_shading: bool, shading_multiplier: f32) -> Color {
            let mut color: Color = WHITE;

            if dark_shading { 
                color.r /= dist * shading_multiplier; 
                color.g /= dist * shading_multiplier; 
                color.b /= dist * shading_multiplier;
            }
            
            color
        }
    }
}