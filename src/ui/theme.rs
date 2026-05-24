use egui::{Color32, Context, FontFamily, FontId, Stroke, Style, Visuals, Rounding};

pub fn setup_custom_fonts(ctx: &Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Use default fonts for now, but we can load custom .ttf here
    // fonts.font_data.insert(
    //     "Inter".to_owned(),
    //     egui::FontData::from_static(include_bytes!("../../assets/Inter-Regular.ttf")),
    // );
    
    // fonts.families
    //     .get_mut(&FontFamily::Proportional)
    //     .unwrap()
    //     .insert(0, "Inter".to_owned());

    ctx.set_fonts(fonts);
}

pub fn setup_dark_theme(ctx: &Context) {
    let mut style: Style = (*ctx.style()).clone();
    
    // Modern Dark Theme Colors
    let bg_color = Color32::from_rgb(20, 20, 24);
    let panel_bg = Color32::from_rgb(30, 30, 36);
    let accent = Color32::from_rgb(88, 101, 242); // Blurple
    let text_main = Color32::from_rgb(230, 230, 230);
    let text_dim = Color32::from_rgb(150, 150, 150);

    let mut visuals = Visuals::dark();
    visuals.window_fill = bg_color;
    visuals.panel_fill = panel_bg;
    visuals.faint_bg_color = panel_bg;
    
    visuals.widgets.noninteractive.bg_fill = panel_bg;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text_main);
    
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(45, 45, 50);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text_main);
    visuals.widgets.inactive.rounding = Rounding::same(6.0);
    
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 68);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.hovered.rounding = Rounding::same(6.0);
    
    visuals.widgets.active.bg_fill = accent;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.active.rounding = Rounding::same(6.0);

    visuals.selection.bg_fill = accent.linear_multiply(0.5);
    visuals.selection.stroke = Stroke::new(1.0, accent);
    
    visuals.window_rounding = Rounding::same(10.0);
    
    style.visuals = visuals;
    ctx.set_style(style);
}
