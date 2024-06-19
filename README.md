# ReDrop

ProjectM (Milkdrop) Music Visualization in Rust.

## TODO

### ReDrop App

- [X] Add Scroll Area
- [X] Show preset without image in square Button (like img)
- [ ] Show presets in a Grid Layout (Responsive)
- [ ] Change preset time interval on nBar (4 beat)  
  160 bpm, 32 bar -> 48s [4 / ( (160 / 60) ) * 32]
- [ ] Set Window default size (persistence !?)

- [ ] Config View:
  - [ ] Save
  - [ ] Reload
  - [ ] Reset (Default)
  - [ ] `open` File explorer
  - [ ] Shortcut table UI

### Player App

- [X] Set switch preset request Callback
- [X] Calculate FPS
- [ ] Sync FPS (Limit)  
  <https://github.com/emilk/egui/issues/1109>  
  <https://github.com/emilk/egui/discussions/342>  
  <https://docs.rs/egui/latest/egui/util/struct.History.html>
- [X] Window Title = ReDrop - { fps } fps - { preset_name }
- [ ] Disable fullscreen on ESCAPE -> (Config -> Shortcut)
- [X] Hide cursor on fullscreen
- [ ] Toggle fullscreen on Double Click
- [ ] Set preset error Callback
  
- [ ] Add projectm_opengl_render_frame_fbo to crate projectM and test it.  
  <https://github.com/projectM-visualizer/projectm/blob/master/src/api/include/projectM-4/render_opengl.h>
- [ ] Add touch (waveform)
- [ ] Add Notification Label (for event)

### Config

- [ ] Add mesh size (32..512)
- [ ] Check for hard and soft cut, preset_locked  
  <https://github.com/projectM-visualizer/projectm/blob/master/src/api/include/projectM-4/parameters.h>
- [ ] Shortcut:
  - [ ] Toggle Fullscreen (F, Double Click)
  - [ ] Disable Fullscreen (ESCAPE)
  - [ ] Next Preset (N)
  - [ ] Prev. Preset (P)
  - [ ] Random Preset (R)
  - [ ] +/- Beat Sensitivity (UP/DOWN)
  - [ ] Preset Speed (LEFT/RIGHT)
  - [ ] Rating (+/-)

### Preset

- [ ] Rating
- [ ] Screenshot Preview Image
- [ ] Mark `warn` (!) if error (stdout) on last play

### Audio

- [ ] Check audio stereo format: cpal vs projectm (pcm), [l,r,l,r,..] or [l,l,..,r,r,..] ?!
  <https://www.reddit.com/r/rust/comments/s0d65g/cpal_capturing_single_channel_out_of_2_channels/>
- [ ] List Audio Devices
- [ ] Audio Buffer Size
- [ ] Calculate audio buffer size with frame rate (fps) (48000hz / 60 fps = 800)
- [ ] Capture system Audio Output  
  <https://github.com/aizcutei/ruhear>

### Video

- [ ] List screen resolutions
- [ ] Aspect Ratio
- [ ] Zoom fullscreen

### IPC Message

- [x] SetPresetDuration (f64)
- [ ] SetBeatSensitivity { value: f64 }

## Future ideas

- Playlist
- Deck (Muliple Output)
- Timeline edition
- Ableton Link (Sync BPM with applications like Ableton Live, Bitwig Studio, NI Tracktor, and more.)  
  <https://github.com/projectM-visualizer/projectm/issues/451>
- Spout (Send output to applications like Resolume, Max, Processing, MadMapper, TouchDesigner and more.)  
  <https://spout.zeal.co/>

## Known bugs

- [ ] Both App crash on `ipc_check` if the other app (window) is closed (icp_channel: err, closed channel).  
  And can't find the channel name if the application is subsequently reopened. (New channel is created..).
- [ ] ReDrop App doesn't receive IPC messages if the window is minimized on taskbar (not rendered)!
- [ ] Player App sometimes crashes:
  > thread 'main' panicked at \.cargo\registry\src\index.crates.io-6f17d22bba15001f\projectm-2.0.1-alpha\src\core.rs:725:13:  
  > Failed to borrow instance
- [ ] Player App crash at load_preset_file (let data) (only one time at the moment):  
  > thread 'main' panicked at src/player_app.rs:216:50:
  > called `Result::unwrap()` on an `Err` value: Error { kind: InvalidData, message: "stream did not contain valid UTF-8" }
  > note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- [ ] ReDrop App: In `show_preset`: Scroll (MouseWheel) not work with `image_hovered`
