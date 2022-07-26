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

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

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
    for _i in 0..200 {
        drops.push(Drop {
            x: (rng.gen_range(0..WIDTH as i32)),
            y: (rng.gen_range(0..HEIGHT as i32)),
            z: (rng.gen_range(1..20)),
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

fn draw(drops: &mut Vec<Drop>, frame: &mut [u8]) {
    let pixels = frame.chunks_exact_mut(4);
    for pixel in pixels {
        pixel.copy_from_slice(&[0xd3, 0xb7, 0xf7, 0xff]);
    }

    drops.iter_mut().for_each(|d| d.update());

    for drop in drops {
        let mut drawn = false;
        for y in drop.y..(drop.y + drop.z) {
            for x in drop.x..(drop.x + drop.z/4) {
                let res = (y * (WIDTH as i32) * 4 + x * 4).try_into();
                if res.is_ok() {
                    let i = res.unwrap();

                    if i < frame.len() {
                        drawn = true;
                        frame[i..i + 4].copy_from_slice(&[207, 100, 219, 0xff]);
                    }
                }
            }
        }
        if !drawn {
            drop.y = -drop.z;
        }
    }
}

struct Drop {
    x: i32,
    y: i32,
    z: i32,
}

impl Drop {
    fn update(&mut self) {
        self.y += std::cmp::max(self.z/4, 1);
    }
}
