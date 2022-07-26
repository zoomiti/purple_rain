use chrono::Duration;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use timer::Timer;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use purple_rain::*;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Purple Rain")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)
    }
    .unwrap();

    let mut drops = Vec::new();
    let mut rng = rand::thread_rng();

    for _i in 0..1000 {
        drops.push(Drop {
            x: (rng.gen_range(0..WIDTH as i32)),
            y: (rng.gen_range(0..HEIGHT as i32)),
            z: ((rng.sample(rand_distr::Geometric::new(0.2).unwrap())+1) as i32),
        })
    }

    let timer = Timer::new();
    let guard = timer.schedule_repeating(Duration::milliseconds(10), move || {
        window.request_redraw();
    });
    guard.ignore();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::RedrawRequested(_) = event {
            draw(&mut drops, pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.key_pressed(VirtualKeyCode::Q) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            } else if input.key_pressed(VirtualKeyCode::Space) {
                //window.request_redraw();
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
        }
    });
}

