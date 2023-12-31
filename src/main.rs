extern crate sdl2;

use binding_of_rust::game::collision::collision_detection;
use binding_of_rust::game::enemy::Enemy;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

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

    let texture_creator = canvas.texture_creator();

    let mut event_pump = match sdl_context.event_pump() {
        Ok(x) => x,
        Err(error) => panic!("Couldn't catch events: {:?}", error),
    };

    let temp_surface =
        sdl2::surface::Surface::load_bmp(Path::new("./sprites/character.bmp")).unwrap();

    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .unwrap();

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

    let mut enemy_list = Vec::new();
    Enemy::add(400, 400, 10, &mut enemy_list);
    Enemy::add(700, 600, 5, &mut enemy_list);

    let mut shooting = false;
    let bullet_speed = 20;

    let mut bullets: Vec<(Direction, Rect)> = Vec::new();
    let mut direction = Direction::Right;

    let gun_cooldown = Duration::from_secs(1);
    let mut previous_shot = Instant::now() - gun_cooldown;

    'running: loop {
        canvas.set_draw_color(Color::RGB(153, 76, 0));
        canvas.clear();
        let _ = canvas.copy(&texture, None, player);

        Enemy::render(&mut enemy_list, &mut canvas);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    if Instant::now() - previous_shot > gun_cooldown {
                        previous_shot = Instant::now();
                        shooting = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    *movement.get_mut("up").unwrap() = true;
                    direction = Direction::Up;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => *movement.get_mut("up").unwrap() = false,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    *movement.get_mut("right").unwrap() = true;
                    direction = Direction::Right;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => *movement.get_mut("right").unwrap() = false,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    *movement.get_mut("down").unwrap() = true;
                    direction = Direction::Down;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => *movement.get_mut("down").unwrap() = false,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    *movement.get_mut("left").unwrap() = true;
                    direction = Direction::Left;
                }
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

        if shooting {
            let bullet = Rect::new(player.center().x(), player.center().y(), 20, 20);
            bullets.push((direction, bullet));
            shooting = false;
        }

        collision_detection(&enemy_list, &mut player);

        for (dir, bullet) in &mut bullets {
            match dir {
                Direction::Up => bullet.set_y(bullet.y() - bullet_speed),
                Direction::Right => bullet.set_x(bullet.x() + bullet_speed),
                Direction::Down => bullet.set_y(bullet.y() + bullet_speed),
                Direction::Left => bullet.set_x(bullet.x() - bullet_speed),
            }
            canvas.set_draw_color(Color::RGB(100, 0, 70));
            canvas.fill_rect(*bullet).unwrap();
            for enemy in &mut enemy_list {
                if enemy.body.has_intersection(*bullet) && enemy.alive {
                    enemy.health -= 1;
                    bullet.set_y(0);
                }
            }
        }
        bullets.retain(|(_, x)| {
            x.x() < window_width && x.x() > 0 && x.y() < window_height && x.y() > 0
        });

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
