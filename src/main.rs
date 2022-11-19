use macroquad::prelude::*;
use map::world::World;
use player::player::Player;
use display::display::Settings;

mod map;
mod player;
mod raycast;
mod display;

/* 
    Simple raycast graphics built from Lode's Computer Graphics Tutorial
    https://lodev.org/cgtutor/raycasting.html
*/

fn conf() -> Conf {
    Conf {
        window_title: String::from("Raycast"),
        window_width: 640,
        window_height: 480,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let world: World = World::new().await;
    let display_settings: Settings = Settings::new(world.textures.len());
    let mut player: Player = Player::new(world, display_settings);
    
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        set_camera(&player.ds.camera);

        /* 
            Drawing textured floors is super slow.
            I did have a super cool fast method using from_rgba8, but as the drop implementation for the texture
            doesn't deallocate memory, it crashes. Will fix later, using slow method for now.
        */
        player.draw_floor();    
        player.draw_sprites();
        player.raycast();   // Walls are drawn here
        player.movement();  // Get player input
        player.ds.draw_ui();
        next_frame().await
    }
}
