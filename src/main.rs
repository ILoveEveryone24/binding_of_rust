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

    let walls = HashMap::from([
        ("top", Rect::new(0, 0, window_width as u32, 200)),
        (
            "right",
            Rect::new(window_width - 200, 0, 200, window_height as u32),
        ),
        (
            "bottom",
            Rect::new(0, window_height - 200, window_width as u32, 200),
        ),
        ("left", Rect::new(0, 0, 200, window_height as u32)),
    ]);

    let rock = Rect::new(400, 400, 50, 50);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(200, 10, 10));
        canvas.fill_rect(player).unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 200));
        canvas.fill_rect(rock).unwrap();
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
            if player.top() < walls["top"].bottom() {
                player.set_y(walls["top"].bottom());
            }
        }
        if movement["right"] {
            player.set_x(player.x() + speed);
            if player.right() > walls["right"].left() {
                player.set_x(walls["right"].left() - player.width() as i32);
            }
        }
        if movement["down"] {
            player.set_y(player.y() + speed);
            if player.bottom() > walls["bottom"].top() {
                player.set_y(walls["bottom"].top() - player.height() as i32);
            }
        }
        if movement["left"] {
            player.set_x(player.x() - speed);
            if player.left() < walls["left"].right() {
                player.set_x(walls["left"].right());
            }
        }

        for val in walls.values() {
            canvas.set_draw_color(Color::RGB(0, 200, 0));
            canvas.fill_rect(*val).unwrap();
        }

        match rock.intersection(player) {
            None => {}
            _ => {
                let collision = rock.intersection(player).unwrap();

                if player.right() > rock.left()
                    && player.left() < rock.left()
                    && collision.width() <= collision.height()
                {
                    player.set_x(player.x() - collision.width() as i32);
                } else if player.left() < rock.right()
                    && player.right() > rock.right()
                    && collision.width() <= collision.height()
                {
                    player.set_x(player.x() + collision.width() as i32);
                }
                if player.top() < rock.bottom()
                    && player.bottom() > rock.bottom()
                    && collision.height() <= collision.width()
                {
                    player.set_y(player.y() + collision.height() as i32);
                } else if player.bottom() > rock.top()
                    && player.top() < rock.top()
                    && collision.height() <= collision.width()
                {
                    player.set_y(player.y() - collision.height() as i32);
                }
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
