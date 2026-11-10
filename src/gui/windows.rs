use eframe::egui::{self, Context};

use crate::{RustyAutoClickerApp, types::InteractionMode};

impl RustyAutoClickerApp {
    pub fn show_hotkeys_window(&mut self, ctx: &Context) {
        let idle = self.is_idle();
        egui::Window::new("快捷键设置")
            .fixed_size(egui::vec2(260f32, 150f32))
            .anchor(egui::Align2::CENTER_CENTER, [0f32, 0f32])
            .collapsible(false)
            .open(&mut self.hotkey_window_open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add_sized(
                            [120.0f32, 32.0f32],
                            egui::widgets::Button::new("开始 / 停止"),
                        )
                        .clicked()
                    {
                        // Allow keybind only if app is not busy
                        if idle {
                            self.mode = InteractionMode::SettingAutoclickKey;
                            self.key_autoclick = None;
                        }
                    };
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.disable();
                        let text: String = if let Some(pressed_keys) = self.key_autoclick {
                            format!("{:}", pressed_keys)
                        } else {
                            "请按下任意键".to_string()
                        };
                        ui.add_sized([130.0f32, 32.0f32], egui::widgets::Button::new(text));
                    });
                });
                ui.horizontal(|ui| {
                    if ui
                        .add_sized(
                            [120.0f32, 32.0f32],
                            egui::widgets::Button::new("确认坐标"),
                        )
                        .on_hover_text("提示: 鼠标左键固定不可更改")
                        .clicked()
                    {
                        // Allow keybind only if app is not busy
                        if idle {
                            self.key_set_coord = None;
                            self.mode = InteractionMode::SettingSetCoordKey;
                        }
                    };
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.disable();
                        let text: String = if let Some(pressed_keys) = self.key_set_coord {
                            format!("{:} / 鼠标左键", pressed_keys)
                        } else {
                            "请按下任意键".to_string()
                        };
                        ui.add_sized([130.0f32, 32.0f32], egui::widgets::Button::new(text));
                    });
                });
                ui.horizontal(|ui| {
                    if ui
                        .add_sized(
                            [120.0f32, 32.0f32],
                            egui::widgets::Button::new("长按模式"),
                        )
                        .clicked()
                    {
                        // Allow keybind only if app is not busy
                        if idle {
                            self.key_hold = None;
                            self.mode = InteractionMode::SettingHoldKey;
                        }
                    };
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.disable();
                        let text: String = if let Some(pressed_keys) = self.key_hold {
                            format!("{:}", pressed_keys)
                        } else {
                            "请按下任意键".to_string()
                        };
                        ui.add_sized([130.0f32, 32.0f32], egui::widgets::Button::new(text));
                    });
                });
            });
    }
}
