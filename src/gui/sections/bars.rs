use eframe::egui;

use crate::{
    RustyAutoClickerApp,
    types::{AppMode, InteractionMode},
};

impl RustyAutoClickerApp {
    pub fn show_topbar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // 顶部菜单栏
            egui::MenuBar::new().ui(ui, |ui| {
                if self.is_autoclicking() {
                    if ui.button(self.autoclick_button_label()).clicked() {
                        self.mode = InteractionMode::Idle;
                    };
                } else {
                    if self.hotkey_window_open || self.is_holding() {
                        ui.disable();
                    }
                    if ui.button(self.autoclick_button_label()).clicked() {
                        self.start_autoclick(0u64);
                    }
                }

                ui.separator();
                ui.label("设置：");

                if ui
                    .add_enabled(
                        !self.is_autoclicking() && !self.is_holding(),
                        egui::Button::new("⌨ 快捷键"),
                    )
                    .clicked()
                {
                    self.hotkey_window_open = true
                };

                ui.separator();
                ui.label("运行模式：");

                self.disable_if_busy(ui);
                ui.selectable_value(&mut self.app_mode, AppMode::Bot, "🖥 极速模式")
                    .on_hover_text("尽可能快地自动连点");
                ui.selectable_value(&mut self.app_mode, AppMode::Humanlike, "😆 模拟人工")
                    .on_hover_text("模拟真实人类点击习惯");
            });
        });
    }

    pub fn show_bottombar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("bottom_panel").show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 5.0;
                    ui.label("基于");
                    ui.hyperlink_to(
                        "rusty-autoclicker",
                        "https://github.com/MrTanoshii/rusty-autoclicker",
                    );
                    ui.label("构建 | 依托于");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                    ui.label("与");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.separator();
                    egui::warn_if_debug_build(ui);
                });
            });
        });
    }
}
