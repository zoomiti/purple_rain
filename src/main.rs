#![windows_subsystem = "windows"]

use log::error;
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;

#[cfg(not(target_arch = "wasm32"))]
use winit::event::VirtualKeyCode;
use winit::{
    dpi::LogicalSize,
    event::{
        Event,
        StartCause::{self, ResumeTimeReached},
    },
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
#[cfg(not(target_arch = "wasm32"))]
use winit_input_helper::WinitInputHelper;

use purple_rain::{Drop, HEIGHT, WIDTH};

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(run());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .enable_time()
            .build()
            .unwrap()
            .block_on(run());
    }
}

async fn run() {
    let event_loop = EventLoop::new();
    #[cfg(not(target_arch = "wasm32"))]
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT));
        let scaled_size = LogicalSize::new(f64::from(WIDTH) * 4.0, f64::from(HEIGHT) * 4.0);
        WindowBuilder::new()
            .with_title("Purple Rain")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("Could not initialize the window")
    };

    #[cfg(all(target_arch = "wasm32", not(debug_assertions)))]
    wasm::create_canvas(&window);
    #[cfg(all(target_arch = "wasm32", debug_assertions))]
    let log_list = wasm::create_log_list(&window);

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new_async(WIDTH, HEIGHT, surface_texture).await
    }
    .expect("Could not initialize the pixels object.");

    let mut drops = Vec::new();
    let mut rng = rand::thread_rng();

    for _i in 0..1000 {
        drops.push(Drop {
            x: (rng.gen_range(0..WIDTH as i32)),
            y: (rng.gen_range(0..HEIGHT as i32)),
            z: ((rng.sample(rand_distr::Geometric::new(0.2).unwrap()) + 1) as i32),
        });
    }

    event_loop.run(move |event, _, control_flow| {
        #[cfg(all(target_arch = "wasm32", debug_assertions))]
        wasm::log_event(&log_list, &event);

        match event {
            Event::RedrawRequested(_) => {
                Drop::draw(&mut drops, pixels.get_frame());
                if pixels
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::NewEvents(StartCause::Init) | Event::NewEvents(ResumeTimeReached { .. }) => {
                *control_flow = ControlFlow::WaitUntil(
                    instant::Instant::now() + instant::Duration::from_millis(1000 / 60),
                );
                window.request_redraw();
            }
            #[cfg(not(target_arch = "wasm32"))]
            Event::MainEventsCleared => window.request_redraw(),
            _ => (),
        }

        #[cfg(not(target_arch = "wasm32"))]
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape)
                || input.key_pressed(VirtualKeyCode::Q)
                || input.quit()
            {
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

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;
    #[cfg(debug_assertions)]
    use winit::event::Event;
    use winit::window::Window;

    #[wasm_bindgen(start)]
    pub fn run() {
        console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        #[allow(clippy::main_recursion)]
        super::main();
    }

    pub fn create_canvas(window: &Window) {
        use winit::platform::web::WindowExtWebSys;
        let canvas = window.canvas();

        canvas.style().set_css_text(
            "background-color: crimson; margin: auto; width: 100%; aspect-ratio: 4 / 3;",
        );
        // Set id so that we can move it later
        canvas.set_id("purple_rain");

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let canvas_div = document
            .get_element_by_id("purple_rain_canvas")
            .unwrap_or(From::from(body)); //Get the div
        canvas_div.append_child(&canvas).unwrap();
    }

    #[cfg(debug_assertions)]
    pub fn create_log_list(window: &Window) -> web_sys::Element {
        create_canvas(window);

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let log_header = document.create_element("h2").unwrap();
        log_header.set_text_content(Some("Event Log"));
        body.append_child(&log_header).unwrap();

        let log_list = document.create_element("ul").unwrap();
        body.append_child(&log_list).unwrap();
        log_list
    }

    #[cfg(debug_assertions)]
    pub fn log_event(log_list: &web_sys::Element, event: &Event<()>) {
        log::debug!("{:?}", event);

        // Getting access to browser logs requires a lot of setup on mobile devices.
        // So we implement this basic logging system into the page to give developers an easy alternative.
        // As a bonus its also kind of handy on desktop.
        if let Event::WindowEvent { event, .. } = &event {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let log = document.create_element("li").unwrap();
            log.set_text_content(Some(&format!("{:?}", event)));
            log_list
                .insert_before(&log, log_list.first_child().as_ref())
                .unwrap();
        }
    }
}
