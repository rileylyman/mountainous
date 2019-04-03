use winit::{EventsLoop, CreationError, Window};
use winit::WindowBuilder;
use winit::dpi::LogicalSize;
const WINDOW_NAME: &'static str = "Example";

#[derive(Debug)]
pub struct WinitState {
    pub events_loop: EventsLoop,
    pub window: Window
}

impl WinitState {

    pub fn new<T: Into<String>>(title: T, size: LogicalSize) -> Result<Self, CreationError> {
        let events_loop = EventsLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(size)
            .build(&events_loop);
       output.map(|window| Self {
           events_loop,
           window,
       })
    }
}

impl Default for WinitState {
    fn default() -> Self {
        Self::new(
            WINDOW_NAME,
            LogicalSize {
                width: 800.0,
                height: 600.0,
            },
        ).expect("Could not create window")
    }
}
