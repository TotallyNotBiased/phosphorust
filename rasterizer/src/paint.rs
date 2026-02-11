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

pub fn draw_wireframe_triangle(p0: Point2D, p1: Point2D, p2: Point2D, color: u32, canvas: &mut Canvas) {
    draw_line(p0, p1, color, canvas);
    draw_line(p1, p2, color, canvas);
    draw_line(p2, p0, color, canvas);
}

pub fn draw_filled_triangle(point_a: Point2D, point_b: Point2D, point_c: Point2D, color: u32, canvas: &mut Canvas) {
    let mut p0 = point_a;
    let mut p1 = point_b;
    let mut p2 = point_c;

    if p1.y < p0.y { std::mem::swap(&mut p1, &mut p0); }
    if p2.y < p0.y { std::mem::swap(&mut p2, &mut p0); }
    if p2.y < p1.y { std::mem::swap(&mut p2, &mut p1); }

    let y0 = p0.y as i32;
    let y1 = p1.y as i32;
    let y2 = p2.y as i32;

    let mut xs0_1 = lerp(y0.into(), p0.x, y1.into(), p1.x);
    let xs1_2 = lerp(y1.into(), p1.x, y2.into(), p2.x);
    let xs0_2 = lerp(y0.into(), p0.x, y2.into(), p2.x);

    if let Some(_) = xs0_1.pop() { }

    let xs0_1_2 = [xs0_1.as_slice(), xs1_2.as_slice()].concat();

    let m = xs0_1_2.len() / 2;

    let mut x_left: Vec<i32> = Vec::new();
    let mut x_right: Vec<i32> = Vec::new();

    if m < xs0_2.len() && xs0_2[m] < xs0_1_2[m] {
        x_left.extend_from_slice(&xs0_2);
        x_right.extend_from_slice(&xs0_1_2);
    } else {
        x_left = xs0_1_2;
        x_right = xs0_2;
    }

    for y in y0..y2 {
        let idx = (y - y0) as usize;
        
        if idx >= x_left.len() || idx >= x_right.len() {
            continue; 
        }

        let xl = x_left[idx];
        let xr = x_right[idx];

        for x in xl..xr {
            canvas.put_pixel(Point2D { x: x as f64, y: y as f64 }, color);
        }
    }
}


