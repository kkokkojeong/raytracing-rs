mod ray;
mod raytracer;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let width = 1280;
    let height = 720;

    let event_loop = EventLoop::new().unwrap();
    let window_builder = WindowBuilder::new()
        .with_title("Raytracing written Rust")
        .with_inner_size(winit::dpi::LogicalSize::new(width, height));
    let window = window_builder.build(&event_loop).unwrap();

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
