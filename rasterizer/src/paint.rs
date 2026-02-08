use cg_common::math::{Point2D, lerp};
use cg_common::canvas::Canvas;

pub fn draw_line(point_a: Point2D, point_b: Point2D, color: u32, canvas: &mut Canvas) {
    let mut p0 = point_a;
    let mut p1 = point_b;

    if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
        if p0.x > p1.x {
            std::mem::swap(&mut p0, &mut p1);
        }
        let ys = lerp(p0.x, p0.y, p1.x, p1.y);
        for x in (p0.x as i32)..(p1.x as i32) {
            canvas.put_pixel(Point2D { x: x as f64, y: ys[(x - p0.x as i32) as usize] as f64}, color);
        }
    } else {
        if p0.y > p1.y {
            std::mem::swap(&mut p0, &mut p1);
        }
        let xs = lerp(p0.y, p0.x, p1.y, p1.x);
        for y in (p0.y as i32)..(p1.y as i32) {
            canvas.put_pixel(Point2D { x: xs[(y - p0.y as i32) as usize] as f64, y: y as f64}, color);
        }
    }
}
