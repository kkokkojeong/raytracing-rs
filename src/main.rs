use raytracing_rs::raytracer::Raytracer;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();

    let width = 1280;
    let height = 720;

    let event_loop = EventLoop::new().unwrap();
    let window_builder = WindowBuilder::new()
        .with_title("Raytracing written Rust")
        .with_inner_size(winit::dpi::LogicalSize::new(width, height));
    let window = window_builder.build(&event_loop).unwrap();

    // ray
    // raytracer
    let ray = Raytracer::new(width, height);
    ray.render();

    // TODO: wgpu
    // ...


    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            },
            _ => ()
        }
    });

}
