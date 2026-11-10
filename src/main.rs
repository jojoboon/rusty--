#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window in release builds on Windows, this blocks stdout.

use eframe::egui;

mod app;
mod defines;
mod gui;
mod types;
mod utils;

use crate::{
    app::RustyAutoClickerApp,
    defines::{APP_NAME, WINDOW_HEIGHT, WINDOW_WIDTH},
    utils::load_icon,
};

/// 尝试加载系统默认中文字体以避免中文乱码
fn setup_chinese_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    #[cfg(target_os = "windows")]
    let font_paths = [
        "C:\\Windows\\Fonts\\msyh.ttc",    // 微软雅黑
        "C:\\Windows\\Fonts\\simhei.ttf",  // 黑体
        "C:\\Windows\\Fonts\\simsun.ttc",  // 宋体
    ];

    #[cfg(target_os = "macos")]
    let font_paths = [
        "/System/Library/Fonts/PingFang.ttc",
        "/Library/Fonts/Arial Unicode.ttf",
    ];

    #[cfg(target_os = "linux")]
    let font_paths = [
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
    ];

    for path in font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "chinese_font".to_owned(),
                std::sync::Arc::new(egui::FontData::from_owned(font_data)),
            );

            // 将中文字体注入到比例字体与等宽字体首位或回退列表
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .push("chinese_font".to_owned());

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("chinese_font".to_owned());
            break;
        }
    }

    ctx.set_fonts(fonts);
}

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe::egui::ViewportBuilder;

    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: ViewportBuilder::default()
            .with_always_on_top()
            .with_decorations(true)
            .with_inner_size(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT))
            .with_resizable(true)
            .with_transparent(true)
            .with_icon(load_icon()),
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        &format!("{} v{}", APP_NAME, env!("CARGO_PKG_VERSION")),
        native_options,
        Box::new(|cc| {
            // 配置中文字体并设置暗色模式
            setup_chinese_fonts(&cc.egui_ctx);
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(RustyAutoClickerApp::new(cc)))
        }),
    ) {
        native_dialog::DialogBuilder::message()
            .set_level(native_dialog::MessageLevel::Error)
            .set_title("图形界面初始化失败")
            .set_text(format!(
                "{e}\n\n\
                由于图形初始化错误，Rusty 鼠标连点器无法启动。\n\n\
                请确认系统已安装兼容的显卡驱动，并支持 Vulkan、Metal 或 DirectX 12。\n\n\
                如果问题依然存在，请尝试更新显卡驱动或在其他设备上运行该程序。"
            ))
            .alert()
            .show()
            .unwrap();
    };
}
