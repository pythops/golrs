use crate::app::App;
use std::{thread, time};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn render(grid_size: u16) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut app = App::new(window, grid_size).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == app.window().id() => {
            match app.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => {
                    let size = app.surface.surface_size;
                    app.resize(size);
                }
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
            thread::sleep(time::Duration::from_millis(500));
        }
        Event::MainEventsCleared => {
            app.window().request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == app.window().id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                app.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                app.resize(**new_inner_size);
            }
            _ => {}
        },
        _ => {}
    });
}
