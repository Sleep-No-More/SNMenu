use std::f64::consts::PI;

/// Represents a single circular button wedge
#[derive(Clone, Debug)]
pub struct CircularButton {
    pub label: String,
    #[allow(dead_code)]
    pub action: String,
    pub color: (f64, f64, f64, f64), // RGBA
    pub hover_color: (f64, f64, f64, f64),
    #[allow(dead_code)]
    pub icon_path: Option<String>, // Path to icon file
}

/// Animation state for smooth transitions
static mut CURRENT_SCALE: [f64; 6] = [1.0; 6];
static mut CURRENT_Y_OFFSET: [f64; 6] = [0.0; 6];
const ANIMATION_SPEED: f64 = 0.01;

/// Calculate which wedge button the user clicked
pub fn get_clicked_button(
    x: f64,
    y: f64,
    center_x: f64,
    center_y: f64,
    radius: f64,
    num_buttons: usize,
    start_angle: f64,
) -> i32 {
    // Calculate distance from center
    let dx = x - center_x;
    let dy = y - center_y;
    let distance = (dx * dx + dy * dy).sqrt();

    // Check if click is within the donut ring
    let inner_radius = radius * 0.4;
    if distance > radius || distance < inner_radius {
        return -1;
    }

    // Calculate angle from center (0 = right, π/2 = down, π = left, 3π/2 = up)
    let mut angle = dy.atan2(dx);

    // Normalize angle to [0, 2π)
    if angle < 0.0 {
        angle += 2.0 * PI;
    }

    // Adjust for start_angle (which is -π/2, pointing up)
    let mut relative_angle = angle - start_angle;

    // Normalize to [0, 2π)
    while relative_angle < 0.0 {
        relative_angle += 2.0 * PI;
    }
    while relative_angle >= 2.0 * PI {
        relative_angle -= 2.0 * PI;
    }

    // Calculate which wedge this falls into
    let wedge_size = (2.0 * PI) / num_buttons as f64;
    let button_index = (relative_angle / wedge_size) as i32;

    // Safety clamp
    button_index.max(0).min(num_buttons as i32 - 1)
}

/// Draw a single donut/ring slice with icon label
fn draw_button_wedge(
    cr: &gtk::gdk::cairo::Context,
    center_x: f64,
    center_y: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    label: &str,
    _icon_path: Option<&str>,
    is_hover: bool,
    base_color: (f64, f64, f64, f64),
    hover_color: (f64, f64, f64, f64),
    scale: f64,
) {
    let mid_angle = (start_angle + end_angle) / 2.0;
    let inner_radius = radius * 0.3;

    // Calculate scaled outer radius (inner stays fixed)
    let scaled_radius = if scale > 1.0 { radius * scale } else { radius };

    // Draw donut ring slice without radial separators - just arcs
    cr.new_path();
    // Outer arc
    cr.arc(center_x, center_y, scaled_radius, start_angle, end_angle);
    // Inner arc (reverse direction to close the path)
    cr.arc_negative(center_x, center_y, inner_radius, end_angle, start_angle);
    cr.close_path();

    // Fill with color
    let color = if is_hover { hover_color } else { base_color };
    cr.set_source_rgba(color.0, color.1, color.2, color.3);
    let _ = cr.fill();

    // No border/outline

    // Draw label text in the center of the button - use Nerd Font symbols
    let text_radius = (scaled_radius + inner_radius) / 2.0;
    let icon_x = center_x + text_radius * mid_angle.cos();
    let icon_y = center_y + text_radius * mid_angle.sin();

    // Map power menu labels to Nerd Font power/system symbols
    // Handles both plain text and emoji+text formats
    let symbol_char = match label {
        // Plain text formats
        "LOCK" | "Lock" => '\u{f023}',     // Nerd Font lock icon
        "SLEEP" | "Sleep" => '\u{f186}',   // Nerd Font moon/sleep icon
        "LOGOUT" | "Logout" => '\u{f08b}', // Nerd Font sign-out icon
        "POWER" | "Power" | "Shutdown" | "SHUTDOWN" => '\u{f011}', // Nerd Font power icon
        "SUSPEND" | "Suspend" => '\u{f04c}', // Nerd Font pause icon (double bar)
        "REBOOT" | "Reboot" => '\u{f021}', // Nerd Font sync/refresh icon
        // Emoji + text formats (from user config)
        s if s.contains("Lock") => '\u{f023}',
        s if s.contains("Hibernate") => '\u{f186}',
        s if s.contains("Logout") => '\u{f08b}',
        s if s.contains("Shutdown") => '\u{f011}',
        s if s.contains("Suspend") => '\u{f04c}',
        s if s.contains("Reboot") => '\u{f021}',
        _ => {
            log::warn!("No match for label: '{}'", label);
            '•'
        }
    };

    // Explicitly select Nerd Font for symbol rendering
    cr.select_font_face(
        "JetBrainsMono Nerd Font",
        gtk::gdk::cairo::FontSlant::Normal,
        gtk::gdk::cairo::FontWeight::Normal,
    );
    // Scale the icon size based on hover state
    let icon_size = 48.0 * scale;
    cr.set_font_size(icon_size);

    // Get text extents for proper centering
    let symbol_str = symbol_char.to_string();
    if let Ok(extents) = cr.text_extents(&symbol_str) {
        let text_x = icon_x - extents.width() / 2.0;
        let text_y = icon_y + extents.height() / 2.0;

        cr.move_to(text_x, text_y);
        cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        let _ = cr.show_text(&symbol_str);
    }
}
/// Draw the complete circular menu as a DONUT/RING with labels
pub fn draw_circular_layout(
    cr: &gtk::gdk::cairo::Context,
    width: i32,
    height: i32,
    buttons: &[CircularButton],
    hover_button: i32,
) {
    let width = width as f64;
    let height = height as f64;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = if width < height { width } else { height } * 0.35; // Increased to 35% for bigger ring
    let start_angle = -PI / 2.0;
    let wedge_size = (2.0 * PI) / buttons.len() as f64;

    // Draw semi-opaque overlay for frosted/blur effect
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.35);
    let _ = cr.paint();

    // Update animation states smoothly (unsafe due to static mut)
    unsafe {
        for (i, _button) in buttons.iter().enumerate() {
            let target_scale = if i as i32 == hover_button { 1.12 } else { 1.0 };

            if CURRENT_SCALE[i] < target_scale {
                CURRENT_SCALE[i] += ANIMATION_SPEED;
                if CURRENT_SCALE[i] > target_scale {
                    CURRENT_SCALE[i] = target_scale;
                }
            } else if CURRENT_SCALE[i] > target_scale {
                CURRENT_SCALE[i] -= ANIMATION_SPEED;
                if CURRENT_SCALE[i] < target_scale {
                    CURRENT_SCALE[i] = target_scale;
                }
            }

            // Animate radial expansion
            let target_y = if i as i32 == hover_button { 8.0 } else { 0.0 };
            if (CURRENT_Y_OFFSET[i] - target_y).abs() > 0.1 {
                CURRENT_Y_OFFSET[i] += (target_y - CURRENT_Y_OFFSET[i]) * 0.05;
            } else {
                CURRENT_Y_OFFSET[i] = target_y;
            }
        }
    }

    // Draw each button wedge
    for (i, button) in buttons.iter().enumerate() {
        let button_start = start_angle + (i as f64 * wedge_size);
        let button_end = button_start + wedge_size;
        let is_hover = i as i32 == hover_button;

        let scale = unsafe { CURRENT_SCALE[i] };

        draw_button_wedge(
            cr,
            center_x,
            center_y,
            radius,
            button_start,
            button_end,
            &button.label,
            button.icon_path.as_deref(),
            is_hover,
            button.color,
            button.hover_color,
            scale,
        );
    }

    // Inner circle is now just empty space (no outline or fill)
}
