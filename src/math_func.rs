use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;
use sdl2::pixels::Color;

pub struct MathFunc {
    values: Vec<f64>,
    next_value_change: f64,
}

impl MathFunc {
    pub fn new() -> Self {
        Self {
            values: vec![0.0],
            next_value_change: 0.0
        }
    }

    pub fn change_slope(&mut self, value: f64) {
        self.next_value_change += value;
    }

    pub fn add_value_from_slope(&mut self) {
        self.values.push(self.values.last().unwrap() + self.next_value_change);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        for (x, value) in self.values.iter().enumerate() {
            canvas.draw_point(Point::new(x as i32, 600 - *value as i32)).unwrap();
        }
    }
}