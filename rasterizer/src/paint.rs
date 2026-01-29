use cg_common::math::{Point2D};
use cg_common::canvas::Canvas;

pub fn draw_line(point_a: Point2D, point_b: Point2D, color: u32, canvas: &mut Canvas) {
    let a = (point_a.y - point_b.y) / (point_a.x - point_b.x);
    let b = point_a.y - a * point_a.x;
    for x in (point_a.x as i32)..(point_b.x as i32) {
        let y = a * (x as f64) + b;

        let p = Point2D {x: (x as f64), y};


        canvas.put_pixel(p, color);
    }
}
