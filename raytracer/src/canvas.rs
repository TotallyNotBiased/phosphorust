use crate::math::Point2D;

pub struct Canvas<'a> {
    pub buffer: &'a mut [u32], 
    pub width: u32,
    pub height: u32,
}

impl<'a> Canvas<'a> {
    pub fn put_pixel(&mut self, p: Point2D, color: u32) {
        let x_norm = (self.width / 2) as f64 + p.x;
        let y_norm = (self.height / 2) as f64 - p.y - 1.0;
        let index = ((self.width as f64 * y_norm) + x_norm) as usize;

        self.buffer[index] = color;
    }
}
