extern crate winit;
extern crate log;
extern crate simple_logger;

#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

mod winit_state;
use winit_state::WinitState;

mod hal_state;
use hal_state::HalState;

mod game_state;
use game_state::LocalState;

use winit::{Event, WindowEvent};

fn main() {
    init_logger_can_panic();
    let mut winit_state = WinitState::default();
    let mut hal_state   = HalState::new(&winit_state.window);
    let mut local_state = LocalState::default();

    loop {
        let inputs = UserInput::poll_events_loop(&mut winit_state.events_loop);
        if inputs.end_requested {
            break;
        }

        local_state.update_from_inputs(inputs);
        if let Err(e) = do_the_render(&mut hal_state, &local_state) {
            panic!("Rendering error {:?}", e);
        }
    }

}

pub fn do_the_render(hal_state: &mut HalState, local_state: &LocalState) -> Result<(), &str> {
    hal_state.draw_clear_frame(local_state.color())
}

fn init_logger_can_panic() -> () {
    simple_logger::init().expect("Failed to init logger");
}
