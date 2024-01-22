use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.32, 0.19, 0.0);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.45, 0.27, 0.0);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
    }
}

