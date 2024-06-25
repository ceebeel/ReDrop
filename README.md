# ReDrop

ProjectM (Milkdrop) Music Visualization in Rust.

## Build and Run

> cargo build -r  
> cargo run -r

## Dependencies

- ProjectM: `projectm`

  > sudo apt install build-essential cmake libgl1-mesa-dev mesa-common-dev libglm-dev

  On Windows, use vcpkg for GLEW:

  > .\vcpkg install glew:x64-windows-static-md

- Video: `egui`, `eframe`
  - > sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
- Audio: `cpal` (TODO: Config Tips audio with ALSA in WSLg)
- File dialogs: `rfd`

## TODO

### ReDrop App

- [ ] Search Preset:
  - [ ] Search Categories
  - [ ] Hide Empty Categories
- [ ] Grid Layout: auto calcul columns
- [ ] Show Flat Preset: Align text to the left
- [ ] Change preset time interval on nBar (4 beat)  
       160 bpm, 32 bar -> 48s [4 / ( (160 / 60) ) * 32]
- [ ] Set Window default size (persistence !?)

#### Playlist View in bottom

- [ ] Add
- [ ] Delete
- [ ] Current Preset (colored frame)
- [ ] Drag and Drop

#### Config View

- [ ] Auto fit content
- [ ] Shortcut table UI
- [ ] List Screen Resolutions
- [ ] List Audio Devices
- [ ] Option: Show presets in flat list or into a grid
- [ ] Option: Hide Categories (folders, subfolders)
- [ ] Option: Hide Cursor in FullScreen
- [ ] Option: Toggle fullscreen on Double Click (One Click for now)

### Player App

- [ ] Sync FPS (Limit)  
       <https://github.com/emilk/egui/issues/1109>  
       <https://github.com/emilk/egui/discussions/342>  
       <https://docs.rs/egui/latest/egui/util/struct.History.html>  
       <https://github.com/projectM-visualizer/frontend-sdl2/blob/master/src/FPSLimiter.cpp>
- [ ] Set projectM loading preset error Callback
- [ ] Add projectm_opengl_render_frame_fbo to crate projectM and test it.  
       <https://github.com/projectM-visualizer/projectm/blob/master/src/api/include/projectM-4/render_opengl.h>
- [ ] Add Notification Label (for event, preset name)
- [ ] Add touch (waveform)

### Config

- [ ] Check for hard and soft cut, preset_locked  
       <https://github.com/projectM-visualizer/projectm/blob/master/src/api/include/projectM-4/parameters.h>
- [ ] Shortcut:
  - [ ] Next Preset (N) (Need Playlist)
  - [ ] Prev. Preset (P) (Need Playlist)
  - [ ] Preset Speed (LEFT/RIGHT) (Check if up/down fps change annimation speed !?)
  - [ ] Rating (+/-)
- [ ] Data validation, before loading
- [ ] `config.toml` in Home directory (.redrop/config.toml) or in current_dir ? (Where to write it by default ?!)

### Preset

- [ ] Metadata (.redrop)
- [ ] Rating
- [ ] Screenshot Preview Image
- [ ] Mark `warn` (!) if error (stdout) on last play

### Audio

- [ ] Check audio stereo format: cpal vs projectm (pcm), [l,r,l,r,..] or [l,l,..,r,r,..] ?!  
       <https://www.reddit.com/r/rust/comments/s0d65g/cpal_capturing_single_channel_out_of_2_channels/>
- [ ] Select Audio Devices
- [ ] Audio Buffer Size
- [ ] Calculate audio buffer size with frame rate (fps) (48000hz / 60 fps = 800)
- [ ] Capture system Audio Output  
       <https://github.com/aizcutei/ruhear>

### Video

- [ ] List screen resolutions
- [ ] Aspect Ratio
- [ ] Zoom fullscreen

## Future ideas

- Playlist
- Deck (Muliple Output)
- Timeline edition
- Ableton Link (Sync BPM with applications like Ableton Live, Bitwig Studio, NI Tracktor, and more.)  
  <https://github.com/projectM-visualizer/projectm/issues/451>
- Spout (Send output to applications like Resolume, Max, Processing, MadMapper, TouchDesigner and more.)  
  <https://spout.zeal.co/>

## Known bugs

- [ ] Both App crash on `check_for_ipc_message` if the other app (window) is closed (icp_channel: err, closed channel).  
       And can't find the channel name if the application is subsequently reopened. (New channel is created..).
- [ ] ReDrop App doesn't receive IPC messages if the window is minimized on taskbar (not rendered)!
- [ ] Player App sometimes crashes:
  > thread 'main' panicked at \.cargo\registry\src\index.crates.io-6f17d22bba15001f\projectm-2.0.1-alpha\src\core.rs:725:13:  
  > Failed to borrow instance
- [ ] ReDrop App: In `show_preset`: Scroll (MouseWheel) not work with `image_hovered`
- [ ] Redrop App: Openned config `FileDialog` block the app and `check_for_ipc_message` in `update`
- [ ] ReDrop App: Can't randomize empty list `send_random_preset_file` (Show Label: "No preset found, check path in config !")
  > thread 'main' panicked at cargo\registry\src\index.crates.io-6f17d22bba15001f\rand-0.8.5\src\rng.rs:134:9:  
  > cannot sample empty range
- [ ] Config UI: `DragValue` speed is not step !
