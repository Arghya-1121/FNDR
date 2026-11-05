use egui::{Vec2, ViewportBuilder};
use fndr::ui::Launcher;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        centered: true,

        viewport: ViewportBuilder::default()
            .with_title("FNDR")
            .with_title_shown(false)
            .with_app_id("com.kolkode.fndr")
            .with_inner_size(Vec2 { x: 600.0, y: 700.0 })
            .with_resizable(false)
            .with_decorations(false)
            .with_taskbar(false)
            .with_has_shadow(true)
            .with_movable_by_background(false)
            .with_close_button(false)
            .with_active(true),

        ..Default::default()
    };

    eframe::run_native(
        "FNDR",
        options,
        Box::new(|_| Ok(Box::new(Launcher::default()))),
    )
}
