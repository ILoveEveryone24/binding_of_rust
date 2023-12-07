extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::time::Duration;

fn main() {
    let window_width: i32 = 2560;
    let window_height: i32 = 1600;
    let sdl_context = match sdl2::init() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't initialize sdl context: {:?}", error),
    };
    let video_subsystem = match sdl_context.video() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't initialize video_subsystem: {:?}", error),
    };
    let window = match video_subsystem
        .window("Binding of rust", window_width as u32, window_height as u32)
        .position_centered()
        .resizable()
        .build()
    {
        Ok(x) => x,
        Err(error) => panic!("Couldn't create a window: {:?}", error),
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't create canvas: {:?}", error),
    };

    let mut event_pump = match sdl_context.event_pump() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't catch events: {:?}", error),
    };

    let mut player = Rect::new(1280, 800, 100, 100);
    let speed = 15;
    let mut movement = HashMap::from([
        ("up", false),
        ("right", false),
        ("down", false),
        ("left", false),
    ]);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(200, 10, 10));
        canvas.fill_rect(player).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => *movement.get_mut("up").unwrap() = true,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => *movement.get_mut("up").unwrap() = false,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => *movement.get_mut("right").unwrap() = true,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => *movement.get_mut("right").unwrap() = false,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => *movement.get_mut("down").unwrap() = true,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => *movement.get_mut("down").unwrap() = false,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => *movement.get_mut("left").unwrap() = true,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => *movement.get_mut("left").unwrap() = false,
                _ => {}
            }
        }
        if movement["up"] {
            player.set_y(player.y() - speed);
            if player.y() < 0 {
                player.set_y(0);
            }
        }
        if movement["right"] {
            player.set_x(player.x() + speed);
            if player.x() > window_width {
                player.set_x(window_width);
            }
        }
        if movement["down"] {
            player.set_y(player.y() + speed);
            if player.y() > window_height {
                player.set_y(window_height);
            }
        }
        if movement["left"] {
            player.set_x(player.x() - speed);
            if player.x() < 0 {
                player.set_x(0);
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
