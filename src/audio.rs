use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, StreamConfig};

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
            .default_input_device()
            .expect("No output device available");

        println!("Using default output device: {:?}", device.name());

        Self {
            device,
            project_m,
            is_capturing: true,
        }
    }

    // TODO: https://github.com/aizcutei/ruhear - capture system output audio
    pub fn capture_audio(&self) {
        let mut config: StreamConfig = self.device.default_input_config().unwrap().into();
        // TODO: Calculate correct buffer size with frame rate (fps)
        // TODO: Fix alsa sample rate (only odd numbers are supported)

        config.buffer_size = BufferSize::Fixed(800); // 48000hz / 60 fps = 800
        println!("Config: {:?}", config);

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
                    // TODO: Check if pcm is [l,r,l,r] !? https://www.reddit.com/r/rust/comments/s0d65g/cpal_capturing_single_channel_out_of_2_channels/
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
            // TODO : Move into the loop !?
            std::thread::sleep(std::time::Duration::from_millis(1)); // TODO: Check if sync with frame rate !
        }
    }
}
