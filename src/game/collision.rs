use super::enemy::Enemy;
use sdl2::rect::Rect;

pub fn collision_detection(enemy_list: &Vec<Enemy>, obj_two: &mut Rect) {
    for enemy in enemy_list {
        if enemy.alive {
            let obj_one = enemy.body;
            match obj_one.intersection(*obj_two) {
                None => {}
                _ => {
                    let collision = obj_one.intersection(*obj_two).unwrap();

                    if obj_two.right() > obj_one.left()
                        && obj_two.left() < obj_one.left()
                        && collision.width() <= collision.height()
                    {
                        obj_two.set_x(obj_two.x() - collision.width() as i32);
                    } else if obj_two.left() < obj_one.right()
                        && obj_two.right() > obj_one.right()
                        && collision.width() <= collision.height()
                    {
                        obj_two.set_x(obj_two.x() + collision.width() as i32);
                    }
                    if obj_two.top() < obj_one.bottom()
                        && obj_two.bottom() > obj_one.bottom()
                        && collision.height() <= collision.width()
                    {
                        obj_two.set_y(obj_two.y() + collision.height() as i32);
                    } else if obj_two.bottom() > obj_one.top()
                        && obj_two.top() < obj_one.top()
                        && collision.height() <= collision.width()
                    {
                        obj_two.set_y(obj_two.y() - collision.height() as i32);
                    }
                }
            }
        }
    }
}
