use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FramerateCounter {
    last_frame_start: Instant,
    prev_frame_start: Instant,
    frame_times: std::collections::VecDeque<Duration>,
    framerate: f64,
    frame_time_target: Duration,
    avg_frame_count: usize,
}

impl FramerateCounter {
    pub fn new(target_fps: u32, avg_frame_count: usize) -> Self {
        FramerateCounter {
            last_frame_start: Instant::now(),
            prev_frame_start: Instant::now(),
            frame_times: std::collections::VecDeque::with_capacity(avg_frame_count),
            framerate: 0.0,
            frame_time_target: Duration::from_nanos(1_000_000_000 / target_fps as u64),
            avg_frame_count,
        }
    }

    pub fn start_frame(&mut self) {
        println!("----- START FRAME -----");
        self.last_frame_start = Instant::now();
    }

    pub fn end_frame(&mut self) {
        let frame_time = self.last_frame_start - self.prev_frame_start;
        self.prev_frame_start = self.last_frame_start;

        self.frame_times.push_back(frame_time);
        if self.frame_times.len() > self.avg_frame_count {
            self.frame_times.pop_front();
        }

        let avg_frame_time: Duration = self.frame_times.iter().sum();
        self.framerate = self.frame_times.len() as f64 / avg_frame_time.as_secs_f64();

        println!("Frame time: {:?}", frame_time);
        println!("Framerate: {} fps", self.framerate);

        let delay = self.frame_time_target.saturating_sub(frame_time);
        println!("Delay: {:?}", delay);
        if delay > Duration::default() {
            println!(" > Sleeping: {:?}", delay);
            std::thread::sleep(delay);
        }
        println!("----- END FRAME -----");
    }
}
// self.framerate = 1.0 / frame_time.as_secs_f64();
// #[derive(Debug)]
// struct FramerateCounter {
//     last_time: Instant,
//     framerate: f64,
//     frame_rate_target: f64,
// }

// impl Default for FramerateCounter {
//     fn default() -> Self {
//         FramerateCounter {
//             last_time: Instant::now(),
//             framerate: 0.0,
//             frame_rate_target: 60.,
//         }
//     }
// }
// impl FramerateCounter {
//     fn update(&mut self) {
//         // println!("-----");
//         let now = Instant::now();
//         let elapsed = now - self.last_time;
//         self.last_time = now;
//         // println!("Elapsed: {:?}", elapsed);

//         self.framerate = 1.0 / elapsed.as_secs_f64();
//         // println!("Framerate: {} fps", self.framerate);

//         let target_delay = Duration::from_secs_f64(1.0 / self.frame_rate_target);
//         // println!("Target delay: {:?}", target_delay);

//         // let delay = target_delay - elapsed;
//         let delay = target_delay.saturating_sub(elapsed);
//         // println!("Delay: {:?}", delay);

//         if delay > Duration::default() {
//             // println!(" > Sleeping: {:?}", delay);
//             std::thread::sleep(delay);
//         }
//     }
// }
// impl FramerateCounter {
//     fn update(&mut self) {
//         let now = Instant::now();
//         let elapsed = now - self.last_time;
//         self.last_time = now;
//         println!("Elapsed: {:?}", elapsed);
//         // if elapsed >= Duration::from_secs(1) {
//         self.framerate = 1. / elapsed.as_secs_f64();
//         println!("Framerate: {} fps", self.framerate);
//         // }

//         let delay = self.frame_rate_target - self.framerate;
//         println!("Delay: {}", delay);
//         if delay > 0.0 {
//             // std::thread::sleep(Duration::from_millis((delay / 1000.) as u64));
//         }
//     }
// fn sync_to_target(&mut self) {
//     let now = Instant::now();
//     let elapsed = now - self.last_time;
//     println!("Elapsed: {:?}", elapsed);
//     let delay = self.frame_rate_target - elapsed.as_millis() as f64;
//     println!("Delay: {} ms", delay);
//     if delay > 0.0 {
//         // std::thread::sleep(Duration::from_millis((delay / 1000.) as u64));
//     }
// }
// }
