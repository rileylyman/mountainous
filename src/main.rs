extern crate winit;

use winit::{Event, WindowEvent};

mod winit_state;
use winit_state::WinitState;

fn main() {
    let mut winit_state = WinitState::default();
    let mut running = true;
    while running {
        winit_state.events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => running = false,
            _ => (),
        });
    }
}
