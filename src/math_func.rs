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
            values: vec![0.0, 1.0],
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

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        let x = self.values.len() - 2;
        draw_slope(canvas, x as f64, self.values[x], self.values[x + 1] - self.values[x], 20.0);
    }
}

fn draw_slope(canvas: &mut Canvas<Window>, x: f64, y: f64, slope: f64, size: f64) {
    canvas.draw_line(Point::new((x - size) as i32, 600 - (y - slope * size) as i32), Point::new((x + size) as i32, 600 - (y + slope * size) as i32)).unwrap();
}
