use common::ipc_message::Message;

use crate::PlayerApp;

impl PlayerApp {
    pub fn check_for_input_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| {
            i.key_pressed(self.config.shortcuts.toggle_fullscreen) || i.pointer.any_click()
        }) {
            // TODO: Fix: any_click() to double click
            self.toggle_fullscreen(ctx);
        }

        if ctx.input(|i| i.key_pressed(self.config.shortcuts.disable_fullscreen)) && self.fullscreen
        {
            self.toggle_fullscreen(ctx);
        }

        if ctx.input(|i| i.key_pressed(self.config.shortcuts.random_preset)) {
            self.send_random_preset_request();
        }

        if ctx.input(|i| i.key_pressed(self.config.shortcuts.beat_sensitivity_up)) {
            self.config.beat_sensitivity += 0.1;
            self.project_m
                .set_beat_sensitivity(self.config.beat_sensitivity);
            self.ipc_to_parent
                .send(Message::SetBeatSensitivity(self.config.beat_sensitivity))
                .unwrap();
            println!("SetBeatSensitivity: {}", self.config.beat_sensitivity);
        }
        if ctx.input(|i| i.key_pressed(self.config.shortcuts.beat_sensitivity_down)) {
            self.config.beat_sensitivity -= 0.1;
            self.project_m
                .set_beat_sensitivity(self.config.beat_sensitivity);
            self.ipc_to_parent
                .send(Message::SetBeatSensitivity(self.config.beat_sensitivity))
                .unwrap();
            println!("SetBeatSensitivity: {}", self.config.beat_sensitivity);
        }
    }

    pub fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        // TODO: Zoom on viewport VS resize viewport (project_m) (maybe ctx.zoom_factor ?!)
        if self.fullscreen {
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::Fullscreen(false));
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::CursorVisible(true));
            self.project_m.set_window_size(
                self.config.window_width as usize,
                self.config.window_height as usize,
            );

            // ctx.set_zoom_factor(1.); // TODO: Fix: zoom not work with project_m
            self.fullscreen = false;
        } else {
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::Fullscreen(true));
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::CursorVisible(false));

            // Resize viewport
            let monitor_size = ctx.input(|i| i.viewport().monitor_size);
            self.project_m.set_window_size(
                monitor_size.unwrap().x as usize,
                monitor_size.unwrap().y as usize,
            );

            // ctx.set_zoom_factor(2.); // TODO: Fix: zoom not work with project_m
            self.fullscreen = true;
        }
    }
}
