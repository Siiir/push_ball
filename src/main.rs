// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.,
                height: 600.,
                title: "Push Ball".to_string(),
                canvas: Some("#bevy".to_owned()),
                ..Default::default()
            },
            ..default()
        }))
        .add_startup_system(set_window_icon)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<bevy::winit::WinitWindows>) {
    let primary = windows.get_window(bevy::window::WindowId::primary()).unwrap();
    let icon_buf = std::io::Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = winit::window::Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
