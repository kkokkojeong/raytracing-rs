use raytracing_rs::raytracer::Raytracer;
use raytracing_rs::state::State;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowBuilder},
};
use winit::event::ElementState;

async fn run() {
    let mut width = 800;
    let mut height = 600;

    let event_loop = EventLoop::new().unwrap();
    let window_builder = WindowBuilder::new()
        .with_title("Raytracing written Rust")
        .with_inner_size(winit::dpi::LogicalSize::new(width, height));
    let window = window_builder.build(&event_loop).unwrap();


    // super-sampling test codes
    width /= 8;
    height /= 8;

    println!("width: {}, height: {}", width, height);

    // image buffer
    let mut img_buff = image::RgbImage::new(width as u32, height as u32);

    // Start ray tracing
    let ray = Raytracer::new(width, height);
    ray.render(&mut img_buff);

    // initialize wgpu
    let mut state = State::new(&window, &mut img_buff).await;

    // State::new uses async code, so we're going to wait for it to finish
    let mut surface_configured = false;

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                            event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                            ..
                        } => elwt.exit(),
                        WindowEvent::Resized(physical_size) => {
                            // log::info!("physical_size: {physical_size:?}");
                            // println!("physical_size: {physical_size:?}");
                            surface_configured = true;
                            state.resize(*physical_size);
                        },
                        WindowEvent::RedrawRequested => {
                            // This tells winit that we want another frame after this one
                            state.window().request_redraw();

                            if !surface_configured {
                                return;
                            }

                            match state.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if it's lost or outdated
                                Err(
                                    wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                ) => state.resize(state.size),
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    log::error!("OutOfMemory");
                                    elwt.exit();
                                }

                                // This happens when the frame takes too long to present
                                Err(wgpu::SurfaceError::Timeout) => {
                                    log::warn!("Surface timeout")
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            _ => ()
        }
    });
}

fn main() {
    env_logger::init();

    pollster::block_on(run());

}
