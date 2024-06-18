# ReDrop

 ProjectM (Milkdrop) Music Visualization in Rust.

## TODO

### ReDrop App

- [ ] Config View:
  - [ ] Save
  - [ ] Reload
  - [ ] Reset (Default)
  - [ ] `open` File explorer
- [ ] Show preset without image in Square Button (like img)
- [ ] Show presets in a Grid Layout (Responsive)
- [ ] Change preset time interval on nBar (4 beat)
  - 160 bpm, 32 bar -> 48s [4 / ( (160 / 60) ) * 32]
- [ ] Fix: The app doesn't receive IPC messages if the window is minimized on taskbar (not rendered)!

### Player App

- [ ] Set change preset request Callback
- [ ] Set preset error Callback
- [ ] Window Title = ReDrop - { fps } fps - { preset_name }
- [ ] Hide cursor on fullscreen
- [ ] Toggle fullscreen on Double Click
- [ ] Calculate & Sync FPS
  - <https://github.com/emilk/egui/issues/1109>
  - <https://github.com/emilk/egui/discussions/342>
  - <https://docs.rs/egui/latest/egui/util/struct.History.html>
- [ ] Add projectm_opengl_render_frame_fbo to crate projectM and test it:
  - <https://github.com/projectM-visualizer/projectm/blob/master/src/api/include/projectM-4/render_opengl.h>
- [ ] Add touch (waveform)

### Config

- [ ] Add mesh size (32..512)
- [ ] Check for hard and soft cut, preset_locked
  - <https://github.com/projectM-visualizer/projectm/blob/master/src/api/include/projectM-4/parameters.h>

### Preset

- [ ] Rating
- [ ] Screenshot Preview Image
- [ ] Mark `warn` (!) if error (stdout) on last play

### Audio

- [ ] List Audio Devices
- [ ] Audio Buffer Size
- [ ] Calculate audio buffer size with frame rate (fps) (48000hz / 60 fps = 800)
- [ ] Capture system Audio Output

### Video

- [ ] List screen resolutions
- [ ] Aspect Ratio
- [ ] Zoom fullscreen

### IPC Message

- [ ] SetBeatSensitivity { value: f64 }
- [ ] SetBeatDuration { value: f32 }
