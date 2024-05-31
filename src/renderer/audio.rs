use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, StreamConfig,
};

use super::ProjectMWrapped;

#[derive(Clone)]
pub struct Audio {
    device: cpal::Device,
    project_m: ProjectMWrapped,
    pub is_capturing: bool,
}

impl Audio {
    pub fn new(project_m: ProjectMWrapped) -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("No output device available");

        Self {
            device,
            project_m,
            is_capturing: true,
        }
    }

    pub fn capture_audio(&self) {
        let mut config: StreamConfig = self.device.default_input_config().unwrap().into();
        // TODO: Calculate correct buffer size with frame rate (fps)
        // TODO: Fix alsa sample rate (only odd numbers are supported)
        config.buffer_size = BufferSize::Fixed(768);

        let err_fn = |err: cpal::StreamError| {
            eprintln!("An error occurred on the output audio stream: {}", err)
        };

        let project_m = std::sync::Arc::clone(&self.project_m);

        let stream = self
            .device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &_| {
                    project_m.pcm_add_float(data.to_vec(), 2);
                },
                err_fn,
                None,
            )
            .unwrap();

        stream.play().unwrap();

        loop {
            if !self.is_capturing {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1)); // TODO: Check if sync with frame rate
        }
    }
}
