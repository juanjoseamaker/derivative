extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod math_func;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("derivative", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut math_function = math_func::MathFunc::new();
    let mut arrow_key_state: f64 = 0.0;
    let mut pause = false;
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    arrow_key_state = 0.1;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    arrow_key_state = -0.1;
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    pause = !pause;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } | Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    arrow_key_state = 0.0;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        if !pause {
            math_function.change_slope(arrow_key_state);
            math_function.add_value_from_slope();
        }

        math_function.draw(&mut canvas);

        canvas.present();
        std::thread::sleep(Duration::from_secs_f32(0.01));
    }
}
