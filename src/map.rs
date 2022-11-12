pub mod world {
    use macroquad::prelude::*;

    pub struct Sprite {
        pub x: f32,
        pub y: f32,
        pub texture: u32,
    }

    pub struct Texture {
        pub texture: Texture2D,
        pub texture_data: Vec<Color>,
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
        pub map: Vec<u32>,
        pub textures: Vec<Texture>,
        pub sprite_map: Vec<Sprite>,
        pub shadows: bool,
        pub dark_shading: bool, 
        pub shading_multiplier: f32, 
        columns: usize,
    }

    impl World {
        pub async fn new() -> World {

            let world_map: Vec<u32> = vec![
                8,8,8,8,8,8,8,8,8,8,8,4,4,6,4,4,6,4,6,4,4,4,6,4,
                8,0,0,0,0,0,0,0,0,0,8,4,0,0,0,0,0,0,0,0,0,0,0,4,
                8,0,3,3,0,0,0,0,0,8,8,4,0,0,0,0,0,0,0,0,0,0,0,6,
                8,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,6,
                8,0,3,3,0,0,0,0,0,8,8,4,0,0,0,0,0,0,0,0,0,0,0,4,
                8,0,0,0,0,0,0,0,0,0,8,4,0,0,0,0,0,6,6,6,0,6,4,6,
                8,8,8,8,0,8,8,8,8,8,8,4,4,4,4,4,4,6,0,0,0,0,0,6,
                7,7,7,7,0,7,7,7,7,0,8,0,8,0,8,0,8,4,0,4,0,6,0,6,
                7,7,0,0,0,0,0,0,7,8,0,8,0,8,0,8,8,6,0,0,0,0,0,6,
                7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,6,0,0,0,0,0,4,
                7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,6,0,6,0,6,0,6,
                7,7,0,0,0,0,0,0,7,8,0,8,0,8,0,8,8,6,4,6,0,6,6,6,
                7,7,7,7,0,7,7,7,7,8,8,4,0,6,8,4,8,3,3,3,0,3,3,3,
                2,2,2,2,0,2,2,2,2,4,6,4,0,0,6,0,6,3,0,0,0,0,0,3,
                2,2,0,0,0,0,0,2,2,4,0,0,0,0,0,0,4,3,0,0,0,0,0,3,
                2,0,0,0,0,0,0,0,2,4,0,0,0,0,0,0,4,3,0,0,0,0,0,3,
                1,0,0,0,0,0,0,0,1,4,4,4,4,4,6,0,6,3,3,0,0,0,3,3,
                2,0,0,0,0,0,0,0,2,2,2,1,2,2,2,6,6,0,0,5,0,5,0,5,
                2,2,0,0,0,0,0,2,2,2,0,0,0,2,2,0,5,0,5,0,0,0,5,5,
                2,0,0,0,0,0,0,0,2,0,0,0,0,0,2,5,0,5,0,5,0,5,0,5,
                1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,
                2,0,0,0,0,0,0,0,2,0,0,0,0,0,2,5,0,5,0,5,0,5,0,5,
                2,2,0,0,0,0,0,2,2,2,0,0,0,2,2,0,5,0,5,0,0,0,5,5,
                2,2,2,2,1,2,2,2,2,2,2,1,2,2,2,5,5,5,5,5,5,5,5,5
            ];

            let sprite_map: Vec<Sprite> = vec![
                Sprite {x: 20.5,    y: 11.5,    texture: 10},
                Sprite {x: 18.5,    y: 4.5,     texture: 10},
            ];

            let textures: Vec<&str> = vec![
                "src/assets/eagle.png", 
                "src/assets/redbrick.png", 
                "src/assets/purplestone.png", 
                "src/assets/greystone.png", 
                "src/assets/bluestone.png", 
                "src/assets/mossy.png", 
                "src/assets/wood.png", 
                "src/assets/colorstone.png",
            ];

            let textures: Vec<Texture> = load_textures(textures).await;

            World { 
                map: world_map, 
                columns: 24,
                textures: textures,
                sprite_map: sprite_map, 
                shadows: true,
                dark_shading: false,
                shading_multiplier: 2.0,
            }
        }
        
        pub fn get(&self, r: i32, c: i32) -> u32 {
            self.map[self.columns * r as usize + c as usize]
        }

        pub fn texture(&self, map: (i32, i32)) -> Texture2D {
            self.textures[(self.get(map.0, map.1) - 1) as usize].texture
        }

        pub fn floor_shading(&self, mut color: Color, height: i32, dist: i32, multiplier: f32) -> Color {
            if self.dark_shading { 
                color.r /= (height - dist) as f32 * multiplier;
                color.g /= (height - dist) as f32 * multiplier;
                color.b /= (height - dist) as f32 * multiplier;
            }

            color
        }

        pub fn wall_shading(&self, side: bool, dist: f32) -> Color {
            let mut color: Color = WHITE;

            if self.dark_shading { 
                color.r /= dist * self.shading_multiplier; 
                color.g /= dist * self.shading_multiplier; 
                color.b /= dist * self.shading_multiplier;
            }
            
            if self.shadows { 
                if side { color.r /= 1.5; color.g /= 1.5; color.b /= 1.5; } 
            }

            color
        }
    }
}





