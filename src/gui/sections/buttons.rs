use eframe::egui;
use rdev::{Button, Key};

use crate::{RustyAutoClickerApp, types::ClickButton};

/// Helper macro to add a selectable value for a keyboard key in the UI.
macro_rules! key_option {
    ($ui:expr,$self:expr, $variant:ident) => {$ui.selectable_value(
            &mut $self.click_btn,
            ClickButton::Key(Key::$variant),
            stringify!($variant),
        );
    };
}

impl RustyAutoClickerApp {
    pub fn show_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label("按键类型");
            if ui
                .add(egui::RadioButton::new(
                    matches!(self.click_btn, ClickButton::Mouse(_)),
                    "鼠标",
                ))
                .clicked()
            {
                self.click_btn = ClickButton::Mouse(Button::Left);
            }
            if ui
                .add(egui::RadioButton::new(
                    matches!(self.click_btn, ClickButton::Key(_)),
                    "键盘",
                ))
                .clicked()
            {
                self.click_btn = ClickButton::Key(Key::Space);
            }

            match self.click_btn {
                ClickButton::Mouse(_) => self.show_mouse_buttons(ui),
                ClickButton::Key(_) => self.show_keyboard_buttons(ui),
            }
        });
    }

    fn show_mouse_buttons(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            egui::ComboBox::from_id_salt("mouse_button")
                .selected_text(match self.click_btn {
                    ClickButton::Mouse(Button::Left) => "左键",
                    ClickButton::Mouse(Button::Right) => "右键",
                    ClickButton::Mouse(Button::Middle) => "中键 (滚轮)",
                    _ => "未知",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.click_btn,
                        ClickButton::Mouse(Button::Left),
                        "左键",
                    );
                    ui.selectable_value(
                        &mut self.click_btn,
                        ClickButton::Mouse(Button::Right),
                        "右键",
                    );
                    // rdev's macOS backend can only simulate Left/Right.
                    if !cfg!(target_os = "macos") {
                        ui.selectable_value(
                            &mut self.click_btn,
                            ClickButton::Mouse(Button::Middle),
                            "中键 (滚轮)",
                        );
                    }
                });
        });
    }

    fn show_keyboard_buttons(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            egui::ComboBox::from_
