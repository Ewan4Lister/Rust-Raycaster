pub mod display {
    use macroquad::prelude::*;
    use crate::player::player::Player;
    use crate::raycast::raycast::Ray;

    /* 
        256×224     //  PSX
        640 x 480   // 	480p
        1280x720    //  720p
        1920x1080   //  1080p
    */

    pub struct Display {
        pub render_target: RenderTarget,
        pub material: Material,
        pub camera: Camera2D,
        pub width: f32,
        pub height: f32,
        pub half: f32,
    }

    impl Display {
        pub fn new() -> Display {
            let render_target = render_target(256, 224); 
            let material = load_material(CRT_VERTEX_SHADER, CRT_FRAGMENT_SHADER, Default::default()).unwrap();
            let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));
            camera.render_target = Some(render_target);

            Display { 
                render_target: render_target, 
                material: material,
                camera: camera,
                width: screen_width(),
                height: screen_height(),
                half: screen_height() / 2.0,
            }
        }

        pub fn draw_walls(&mut self, player: &Player, ray: Ray, x: f32) {
            let c: Color = if player.nightvision { GREEN } else { player.world.wall_shading(ray.side, ray.perp_wall_dist) };
            let t: Texture2D = player.world.texture(ray.map);
        
            let line_height: f32 = player.height / ray.perp_wall_dist;
            let  draw_start: f32 = -line_height / 2.0 + player.height / 2.0;

            let mut wall_x: f32;
            if !ray.side { wall_x = player.pos.y + ray.perp_wall_dist * ray.ray_dir.y; }
            else         { wall_x = player.pos.x + ray.perp_wall_dist * ray.ray_dir.x; }
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

        pub fn draw_ui(&mut self) {
            draw_text(
                format!("{} FPS", get_fps()).as_str(),
                10.0,
                66.0,
                25.0,
                GREEN,
            );

            set_default_camera();
            gl_use_material(self.material);
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

        // https://www.shadertoy.com/view/XtlSD7

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


