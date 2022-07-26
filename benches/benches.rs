use bencher::Bencher;
use pixels::Pixels;
use pixels::SurfaceTexture;
use rand::Rng;

#[macro_use]
extern crate bencher;

use purple_rain::draw;
use purple_rain::Drop;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 300;

fn ten_drops(bench: &mut Bencher) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    window.set_visible(false);
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

    bench.iter(|| {
        draw(&mut drops, pixels.get_frame());
    })
}

fn one_hundred_drops(bench: &mut Bencher) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    window.set_visible(false);
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)
    }
    .unwrap();

    let mut drops = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..2000 {
        drops.push(Drop {
            x: (rng.gen_range(0..WIDTH as i32)),
            y: (rng.gen_range(0..HEIGHT as i32)),
            z: (rng.gen_range(1..20)),
        })
    }

    bench.iter(|| {
        draw(&mut drops, pixels.get_frame());
    })
}

benchmark_group!(drops, ten_drops, one_hundred_drops);
benchmark_main!(drops);
