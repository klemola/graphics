mod camera;
mod entity;
mod model;
mod state;
mod steering;
mod texture;

use cgmath::prelude::*;
use state::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Fullscreen,
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();

    // TODO: chain Option values all the way to Option<Fullscreen>
    let monitor = event_loop.available_monitors().next().unwrap();
    let video_mode = monitor.video_modes().next().unwrap();
    let window = WindowBuilder::new()
        .with_fullscreen(Some(Fullscreen::Exclusive(video_mode)))
        .build(&event_loop)
        .unwrap();

    // Since main can't be async, we're going to need to block
    let mut state = pollster::block_on(State::new(&window));
    let mut last_render_time = std::time::Instant::now();

    state.add_cube(
        1,
        cgmath::Vector3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        },
        cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
    );

    state.add_cube(
        2,
        cgmath::Vector3 {
            x: 0.0,
            y: -3.0,
            z: 0.0,
        },
        cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
    );

    state.add_cube(
        3,
        cgmath::Vector3 {
            x: 3.0,
            y: 0.0,
            z: 0.0,
        },
        cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
    );

    state.add_cube(
        4,
        cgmath::Vector3 {
            x: -3.0,
            y: 0.0,
            z: 0.0,
        },
        cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::DeviceEvent { ref event, .. } => {
                state.input(event);
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },

                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to derefence it twice
                        state.resize(**new_inner_size)
                    }

                    _ => {}
                }
            }

            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let dt = now - last_render_time;

                last_render_time = now;
                state.update(dt);

                match state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }

            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it
                window.request_redraw();
            }

            _ => {}
        }
    });
}
