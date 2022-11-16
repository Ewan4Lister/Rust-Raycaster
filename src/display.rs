pub mod display {
    use macroquad::prelude::*;
    use crate::player::player::Player;
    use crate::raycast::raycast::Ray;
    use macroquad::ui::{
        hash, root_ui,
        widgets::{self, Group},
        Drag, Ui,
    };

    /* 
        Display settings, UI and Shaders

        Todo add UI dropdown to change resolutions
        256×224     //  PSX
        640x480     // 	480p
        1280x720    //  720p
        1920x1080   //  1080p
    */
    pub struct Display {
        pub render_target: RenderTarget,
        pub CRT_material: Material,
        pub camera: Camera2D,
        pub width: f32,
        pub height: f32,
        pub half: f32,
        // Settings
        pub settings: bool, 
        pub shaders: bool,   
        pub nightvision: bool,   
        pub fast_floors: bool,
        pub shadows: bool,
        pub dark_shading: bool, 
        pub wall_shading_multiplier: f32, 
        pub floor_shading_multiplier: f32, 
        pub ceil_shading_multiplier: f32, 
    }

    impl Display {
        pub fn new() -> Display {
            let render_target = render_target(640, 480); 
            let CRT_material = load_material(CRT_VERTEX_SHADER, CRT_FRAGMENT_SHADER, Default::default()).unwrap();
            
            let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));
            camera.render_target = Some(render_target);

            Display { 
                render_target: render_target, 
                CRT_material: CRT_material,
                camera: camera,
                width: screen_width(),
                height: screen_height(),
                half: screen_height() / 2.0,

                settings: false,
                shaders: false,
                nightvision: false,
                fast_floors: false,
                shadows: true,
                dark_shading: false,
                wall_shading_multiplier: 2.0,
                floor_shading_multiplier: 0.08,
                ceil_shading_multiplier: 0.1,
            }
        }

        pub fn draw_ui(&mut self) {
            if self.settings {
                draw_text(
                    format!("{} FPS", get_fps()).as_str(),
                    10.0,
                    66.0,
                    25.0,
                    GREEN,
                );

                widgets::Window::new(hash!(), vec2(20., 70.), vec2(300., 300.))
                    .label("Settings")
                    .ui(&mut *root_ui(), |ui| {
                        ui.tree_node(hash!(), "Textures", |ui| {
                            if ui.button(None, "Shaders") {
                                self.shaders = !self.shaders;
                            }
                            ui.separator();

                            if ui.button(None, "Nightvision") {
                                self.nightvision = !self.nightvision;
                            }
                            ui.separator();

                            if ui.button(None, "Textured Floors") {
                                self.fast_floors = !self.fast_floors;
                            }
                            ui.separator();

                            if ui.button(None, "Shadows") {
                                self.shadows = !self.shadows;
                            }
                            ui.separator();
                            
                            if ui.button(None, "Darkness Shading") {
                                self.dark_shading = !self.dark_shading;
                            }

                            if self.dark_shading {
                                ui.label(None,"Wall Darkness");
                                ui.slider(hash!(), "[1.0 .. 25]", 1.0f32..25f32, &mut self.wall_shading_multiplier);
                                ui.label(None,"Floor Darkness");
                                ui.slider(hash!(), "[0.05 .. 1.0]", 0.05f32..1.0f32, &mut self.floor_shading_multiplier);
                                ui.label(None,"Ceil Darkness");
                                ui.slider(hash!(), "[0.05 .. 1.0]", 0.05f32..1.0f32, &mut self.ceil_shading_multiplier);
                            }
                        });           
                        
                        ui.tree_node(hash!(), "Player", |ui| {
                            ui.label(None, "Todo :)")
                        });                  
                    });
            }
            
            set_default_camera();
            if self.shaders { 
                gl_use_material(self.CRT_material); 
            }
            draw_texture_ex(
                self.render_target.texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    flip_y: true,
                    ..Default::default()
                },
            );
            gl_use_default_material();
        }
    }

    const CRT_FRAGMENT_SHADER: &'static str = 
    r#"#version 100
        precision lowp float;

        varying vec4 color;
        varying vec2 uv;

        uniform sampler2D Texture;

        vec2 CRTCurveUV(vec2 uv)
        {
            uv = uv * 2.0 - 1.0;
            vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
            uv = uv + uv * offset * offset;
            uv = uv * 0.5 + 0.5;
            return uv;
        }

        void DrawVignette( inout vec3 color, vec2 uv )
        {    
            float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
            vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
            color *= vignette;
        }


        void DrawScanline( inout vec3 color, vec2 uv )
        {
            float iTime = 0.1;
            float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
            float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );    
            color *= scanline * grille * 1.2;
        }

        void main() {

            vec2 crtUV = CRTCurveUV(uv);

            vec3 res = texture2D(Texture, uv).rgb * color.rgb;
        
            if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
            {
                res = vec3(0.0, 0.0, 0.0);
            } 
            DrawVignette(res, crtUV);
            DrawScanline(res, uv);
            gl_FragColor = vec4(res, 1.0);
        }
    "#;

    const CRT_VERTEX_SHADER: &'static str = 
    "#version 100
        attribute vec3 position;
        attribute vec2 texcoord;
        attribute vec4 color0;

        varying lowp vec2 uv;
        varying lowp vec4 color;

        uniform mat4 Model;
        uniform mat4 Projection;

        void main() {
            gl_Position = Projection * Model * vec4(position, 1);
            color = color0 / 255.0;
            uv = texcoord;
        }
        ";
    }


