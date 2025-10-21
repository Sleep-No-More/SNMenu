mod circular_layout;
mod config;

use anyhow::Result;
use circular_layout::{draw_circular_layout, get_clicked_button, CircularButton};
use config::{load_config, parse_color_with_alpha, Button};
use gtk::prelude::*;
use gtk::{DrawingArea, EventBox, Window, WindowType};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::process::Command;
use std::rc::Rc;
use std::time::Duration;

struct AppState {
    buttons: Vec<Button>,
    hover_button: i32,
    animation_progress: f64, // 0.0 to 1.0 for slide-in
    start_x: f64,
    start_y: f64,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init()
        .ok();

    gtk::init().expect("Failed to initialize GTK");
    build_ui();
    gtk::main();
}

fn get_layout_path() -> Result<String> {
    // Try XDG_CONFIG_HOME first
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        let path = format!("{}/cpmenu/layout", xdg_config);
        if std::path::Path::new(&path).exists() {
            return Ok(path);
        }
    }

    // Try ~/.config/cpmenu/layout
    if let Ok(home) = std::env::var("HOME") {
        let path = format!("{}/.config/cpmenu/layout", home);
        if std::path::Path::new(&path).exists() {
            return Ok(path);
        }
    }

    // Try system paths
    if std::path::Path::new("/etc/cpmenu/layout").exists() {
        return Ok("/etc/cpmenu/layout".to_string());
    }

    if std::path::Path::new("/usr/local/etc/cpmenu/layout").exists() {
        return Ok("/usr/local/etc/cpmenu/layout".to_string());
    }

    Err(anyhow::anyhow!("Failed to find layout file"))
}

fn get_css_path() -> Option<String> {
    // Try XDG_CONFIG_HOME first
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        let path = format!("{}/cpmenu/style.css", xdg_config);
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }

    // Try ~/.config/cpmenu/style.css
    if let Ok(home) = std::env::var("HOME") {
        let path = format!("{}/.config/cpmenu/style.css", home);
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }

    // Try system paths
    if std::path::Path::new("/etc/cpmenu/style.css").exists() {
        return Some("/etc/cpmenu/style.css".to_string());
    }

    if std::path::Path::new("/usr/local/etc/cpmenu/style.css").exists() {
        return Some("/usr/local/etc/cpmenu/style.css".to_string());
    }

    None
}

fn load_css(css_path: Option<String>) {
    if let Some(path) = css_path {
        let provider = gtk::CssProvider::new();
        match provider.load_from_path(&path) {
            Ok(_) => {
                let screen = gdk::Screen::default().expect("Failed to get screen");
                gtk::StyleContext::add_provider_for_screen(
                    &screen,
                    &provider,
                    gtk::STYLE_PROVIDER_PRIORITY_USER,
                );
            }
            Err(e) => {
                log::warn!("Failed to load CSS: {:?}", e);
            }
        }
    }
}

fn get_mouse_position() -> (f64, f64) {
    use std::process::Command;

    // Use hyprctl to get mouse position from Hyprland
    if let Ok(output) = Command::new("hyprctl").args(&["cursorpos"]).output() {
        if let Ok(output_str) = String::from_utf8(output.stdout) {
            let coords: Vec<&str> = output_str.trim().split(',').collect();
            if coords.len() == 2 {
                if let (Ok(x), Ok(y)) = (
                    coords[0].trim().parse::<f64>(),
                    coords[1].trim().parse::<f64>(),
                ) {
                    return (x, y);
                }
            }
        }
    }

    // Fallback to GTK method
    if let Some(display) = gdk::Display::default() {
        if let Some(seat) = display.default_seat() {
            if let Some(pointer) = seat.pointer() {
                let (_screen, x, y) = pointer.position();
                // Only use GTK position if it's not at screen center (more likely to be actual cursor)
                if x != 1920 && y != 1440 {
                    return (x as f64, y as f64);
                }
            }
        }
    }
    (0.0, 0.0)
}

fn build_ui() {
    // Load layout
    let layout_path = match get_layout_path() {
        Ok(path) => path,
        Err(e) => {
            log::error!("Failed to find layout: {}", e);
            return;
        }
    };

    let buttons = match load_config(&layout_path) {
        Ok(buttons) => buttons,
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            return;
        }
    };

    // Load CSS
    let css_path = get_css_path();
    load_css(css_path);

    // Get mouse position for slide-in animation
    let (mouse_x, mouse_y) = get_mouse_position();

    let state = Rc::new(RefCell::new(AppState {
        buttons,
        hover_button: -1,
        animation_progress: 0.0,
        start_x: mouse_x,
        start_y: mouse_y,
    }));

    // Create main window
    let window = Window::new(WindowType::Toplevel);
    window.fullscreen();
    window.set_keep_above(true);
    window.set_decorated(false);

    // Create drawing area for circular menu
    let drawing_area = DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);
    drawing_area.set_events(
        gdk::EventMask::POINTER_MOTION_MASK
            | gdk::EventMask::BUTTON_PRESS_MASK
            | gdk::EventMask::KEY_PRESS_MASK,
    );

    let state_draw = state.clone();
    drawing_area.connect_draw(move |widget, cr| {
        let mut state = state_draw.borrow_mut();

        // Update animation progress (slide in over ~6 frames)
        if state.animation_progress < 1.0 {
            state.animation_progress += 0.12;
            if state.animation_progress > 1.0 {
                state.animation_progress = 1.0;
            }
        }

        let width = widget.allocated_width() as f64;
        let height = widget.allocated_height() as f64;

        // Calculate animation offset (slide from button to center)
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let progress = state.animation_progress;

        let anim_x = state.start_x + (center_x - state.start_x) * progress;
        let anim_y = state.start_y + (center_y - state.start_y) * progress;
        let opacity = progress; // Also fade in

        // Apply animation to the menu itself
        let _ = cr.save();
        cr.translate(anim_x - center_x, anim_y - center_y);

        // Set semi-transparent background with opacity
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.35 * opacity);
        let _ = cr.paint();

        // Convert buttons to CircularButton format
        let circular_buttons: Vec<CircularButton> = state
            .buttons
            .iter()
            .map(|btn| {
                // Use red/coral for power/shutdown button, light blue for others
                let (base_color, hover_color) = if btn.label.contains("power")
                    || btn.label.contains("shutdown")
                    || btn.text.contains("Shutdown")
                    || btn.text.contains("Power")
                {
                    // Power button gets the red/coral color - lighter at rest, darker on hover
                    (
                        parse_color_with_alpha("#E07070", 0.8 * opacity),
                        parse_color_with_alpha("#DC5050", 0.9 * opacity),
                    )
                } else {
                    // Other buttons get light blue base
                    (
                        parse_color_with_alpha("#81A1C1", 0.35 * opacity),
                        parse_color_with_alpha("#5E81AC", 0.55 * opacity),
                    )
                };

                // Try to find icon in icons folder
                let icon_path = [
                    format!("./icons/{}.png", btn.label),
                    format!("/usr/local/share/cpmenu/icons/{}.png", btn.label),
                    format!("/usr/share/cpmenu/icons/{}.png", btn.label),
                ]
                .iter()
                .find(|p| std::path::Path::new(p).exists())
                .cloned();

                CircularButton {
                    label: btn.text.clone(),
                    action: btn.action.clone(),
                    color: base_color,
                    hover_color,
                    icon_path,
                }
            })
            .collect();

        // Draw circular layout
        draw_circular_layout(
            cr,
            width as i32,
            height as i32,
            &circular_buttons,
            state.hover_button,
        );

        let _ = cr.restore();

        false.into()
    });

    let state_motion = state.clone();
    drawing_area.connect_motion_notify_event(move |widget, event| {
        let width = widget.allocated_width() as f64;
        let height = widget.allocated_height() as f64;
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let radius = if width < height { width } else { height } * 0.35;

        let clicked = get_clicked_button(
            event.position().0,
            event.position().1,
            center_x,
            center_y,
            radius,
            state_motion.borrow().buttons.len(),
            -PI / 2.0,
        );

        let mut state = state_motion.borrow_mut();
        if clicked != state.hover_button {
            state.hover_button = clicked;
            widget.queue_draw();
        }

        // Update cursor based on hover state
        if let Some(window) = widget.window() {
            let cursor = if clicked >= 0 {
                gdk::Cursor::from_name(widget.display().as_ref(), "hand")
                    .or_else(|| gdk::Cursor::from_name(widget.display().as_ref(), "pointer"))
            } else {
                None
            };
            window.set_cursor(cursor.as_ref());
        }

        false.into()
    });
    let state_click = state.clone();
    let window_clone = window.clone();
    drawing_area.connect_button_press_event(move |widget, event| {
        let width = widget.allocated_width() as f64;
        let height = widget.allocated_height() as f64;
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let radius = if width < height { width } else { height } * 0.35;

        let clicked = get_clicked_button(
            event.position().0,
            event.position().1,
            center_x,
            center_y,
            radius,
            state_click.borrow().buttons.len(),
            -PI / 2.0,
        );

        if clicked >= 0 {
            let state = state_click.borrow();
            if let Some(button) = state.buttons.get(clicked as usize) {
                log::info!("Executing action: {}", button.action);
                execute_command(&button.action);
            }
        }

        // Hide window immediately on any click (button or empty area)
        window_clone.hide();

        // Queue quit to happen immediately after event processing
        glib::idle_add_once(|| {
            gtk::main_quit();
        });

        true.into()
    });

    // Event box for background clicks
    let event_box = EventBox::new();
    event_box.add(&drawing_area);

    window.add(&event_box);

    // Add animation timer for smooth hover transitions
    let drawing_area_clone = drawing_area.clone();
    glib::timeout_add_local(Duration::from_millis(16), move || {
        drawing_area_clone.queue_draw();
        glib::ControlFlow::Continue
    });

    let window_clone = window.clone();
    window.connect_key_press_event(move |_, key| match key.keyval() {
        gtk::gdk::keys::constants::Escape => {
            window_clone.hide();
            glib::idle_add_once(|| {
                gtk::main_quit();
            });
            true.into()
        }
        _ => false.into(),
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        false.into()
    });

    window.show_all();
}

fn execute_command(command: &str) {
    if let Err(e) = Command::new("sh").arg("-c").arg(command).spawn() {
        log::error!("Failed to execute command: {} - {}", command, e);
    }
    // Don't wait - let command run in background and close menu immediately
}
