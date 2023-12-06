extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    let sdl_context = match sdl2::init() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't initialize sdl context: {:?}", error),
    };
    let video_subsystem = match sdl_context.video() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't initialize video_subsystem: {:?}", error),
    };
    let window = match video_subsystem
        .window("Binding of rust", 2560, 1600)
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
                /*
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    player.set_y(player.y() - speed);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    player.set_x(player.x() - speed);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    player.set_y(player.y() + speed);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    player.set_x(player.x() + speed);
                },*/
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        match keyboard_state.pressed_scancodes().next() {
            Some(Scancode::W) => {
                player.set_y(player.y() - speed);
            }
            Some(Scancode::A) => {
                player.set_x(player.x() - speed);
            }
            Some(Scancode::S) => {
                player.set_y(player.y() + speed);
            }
            Some(Scancode::D) => {
                player.set_x(player.x() + speed);
            }
            _ => {}
        };

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
