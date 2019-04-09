extern crate arrayvec;

use arrayvec::ArrayVec;
use gfx_hal::pso::PipelineStage;
use gfx_hal::queue::Submission;
use gfx_hal::command::{ClearValue, ClearColor};

pub struct HalState {
}

impl HalState {
    pub fn new(win: &super::WinitState) -> Self {
        unimplemented!()
    }

    pub fn draw_clear_frame(&mut self, color: [f32; 4]) -> Result<(), &'static str> {

        let image_available = &self.image_available_semaphores[self.current_frame];
        let render_finished = &self.render_finsihed_semaphores[self.current_frame];

        let self.current_frame = (self.current_frame + 1) % self.frames_in_flight;

        let swapchain_index = unsafe {
            self.swapchain
                .acquire_image(core::u64::MAX, FrameSync::Semaphore(image_available))
                .map_err(|_| "Could not acquire an image from the swapchain!")?
        };

        let flight_fence = self.in_flight_fences[swapchain_index];
        unsafe {
            self.device
                .wait_for_fence(flight_fence, core::u64::MAX)
                .map_err(|_| "Failed to wait on flight fence!")?;
            self.device
                .reset_fence(flight_fence)
                .map_err(|_| "Could not reset flight fence!")?;
        }

        unsafe {
            /* We now record the correct command to one of our pre-built command-buffers.
             * To do this, we first get an iterator of our clear values. (Will use depth
             * stencil later.) Then, we `begin` recording the command, call the appropriate 
             * method to record our values, and then `finish`.*/
            let buffer = &mut self.command_buffers[swapchain_index as usize];
            let clear_values = [ClearValue::Color(ClearColor::Float(color))];
            buffer.begin(false);
            buffer.begin_render_pass_inline(
                &self.render_pass,
                &self.swapchain_framebuffers[swapchain_index as usize],
                self.render_area,
                clear_values.iter(),
                );
            buffer.finish();
        }

        //SUBMISSION
        let command_buffers: ArrayVec<[_;1]> = [clear_command].into();
        let wait_semaphores: ArrayVec<[_;1]> = [(image_available, PipelineStage::COLOR_OUTPUT_ATTACHMENT)].into();
        let signal_semaphores: ArrayVec<[_;1]> = [render_finished].into();
        let present_wait_semaphores: ArrayVec<[_;1]> = [render_finished].into();

        let submission = Submission {
            command_buffers,
            wait_semaphores,
            signal_semaphores,
        };

        unsafe {
            command_queue.submit(submission, Some(flight_fence));
            swapchain.present(command_queue, swapchain_index, present_wait_semaphores).map_err(|_| "Failed to present!");
        }
    }
}
