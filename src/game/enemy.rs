use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Enemy {
    pub alive: bool,
    pub body: Rect,
    pub health: i32,
}

impl Enemy {
    pub fn add(x: i32, y: i32, hp: i32, enemy_list: &mut Vec<Enemy>) {
        let enemy = Enemy {
            alive: true,
            body: Rect::new(x, y, 50, 50),
            health: hp,
        };
        enemy_list.push(enemy);
    }

    pub fn render(enemy_list: &mut Vec<Enemy>, canvas: &mut sdl2::render::WindowCanvas) {
        enemy_list.retain(|x| x.alive);
        for enemy in enemy_list {
            if enemy.alive {
                canvas.set_draw_color(Color::RGB(0, 0, 200));
                canvas.fill_rect(enemy.body).unwrap();
            }
            if enemy.health <= 0 {
                enemy.alive = false;
            }
        }
    }
}
