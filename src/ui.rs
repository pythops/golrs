use crate::app::App;
use std::{thread, time};
use winit::keyboard::PhysicalKey::Code;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::WindowBuilder,
};

pub async fn render(grid_size: u16) {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut app = App::new(&window, grid_size).await;

    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop
        .run(|event, elwt| match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::RedrawRequested => {
                    match app.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            let size = app.surface.surface_size;
                            app.resize(size);
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                    thread::sleep(time::Duration::from_millis(50));
                }
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state.is_pressed() && event.physical_key == Code(KeyCode::Escape) {
                        elwt.exit()
                    }
                }

                WindowEvent::Resized(physical_size) => {
                    app.resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    app.resize(window.inner_size());
                }
                _ => {}
            },
            Event::AboutToWait => {
                app.window().request_redraw();
            }
            _ => {}
        })
        .unwrap();
}
