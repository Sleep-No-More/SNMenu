use std::cell::RefCell;
use std::f64::consts::PI;

// Animation configuration constants.
const INNER_RADIUS_RATIO: f64 = 0.3;
const MENU_RADIUS_PERCENT: f64 = 0.35;
const ICON_SIZE: f64 = 48.0;
const LABEL_OFFSET: f64 = 40.0;
const LABEL_FONT_SIZE: f64 = 14.0;
const LABEL_FONT_ALPHA: f64 = 0.9;
const HOVER_Y_OFFSET: f64 = 8.0;
const HOVER_SCALE: f64 = 1.12;
const ANIMATION_SPEED: f64 = 0.01;

// Thread-local animation state for smooth transitions.
thread_local! {
    static CURRENT_SCALE: RefCell<Vec<f64>> = RefCell::new(vec![1.0; 6]);
    static CURRENT_Y_OFFSET: RefCell<Vec<f64>> = RefCell::new(vec![0.0; 6]);
}

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
    pub icon_char: Option<char>, // Custom icon character (Unicode/Nerd Font)
    pub show_label: bool,        // Whether to show text label
}

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
    icon_char: Option<char>,
    show_label: bool,
) {
    let mid_angle = (start_angle + end_angle) / 2.0;
    let inner_radius = radius * INNER_RADIUS_RATIO;

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

    // Use custom icon_char if provided, otherwise use a generic default
    let symbol_char = icon_char.unwrap_or_else(|| {
        // If no custom icon is provided, use a generic bullet point
        // Users should specify icon_char in their config for any button
        log::debug!(
            "No icon specified for label: '{}', using default bullet",
            label
        );
        '•'
    });

    // Draw the icon - use FiraCode Nerd Font which is available
    cr.select_font_face(
        "FiraCode Nerd Font",
        gtk::gdk::cairo::FontSlant::Normal,
        gtk::gdk::cairo::FontWeight::Normal,
    );
    // Scale the icon size based on hover state
    let icon_size = ICON_SIZE * scale;
    cr.set_font_size(icon_size);

    // Get text extents for proper centering
    let symbol_str = symbol_char.to_string();
    match cr.text_extents(&symbol_str) {
        Ok(extents) => {
            let text_x = icon_x - extents.width() / 2.0;
            let text_y = icon_y + extents.height() / 2.0;

            cr.move_to(text_x, text_y);
            cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
            let _ = cr.show_text(&symbol_str);
        }
        Err(e) => {
            log::warn!("Failed to render icon: {:?}", e);
        }
    }

    // Draw button label text below the icon (only if show_label is true)
    if !show_label {
        return;
    }

    cr.select_font_face(
        "Sans",
        gtk::gdk::cairo::FontSlant::Normal,
        gtk::gdk::cairo::FontWeight::Normal,
    );
    cr.set_font_size(LABEL_FONT_SIZE);
    cr.set_source_rgba(1.0, 1.0, 1.0, LABEL_FONT_ALPHA);

    match cr.text_extents(label) {
        Ok(label_extents) => {
            let label_x = icon_x - label_extents.width() / 2.0;
            let label_y = icon_y + LABEL_OFFSET;

            cr.move_to(label_x, label_y);
            let _ = cr.show_text(label);
        }
        Err(e) => {
            log::warn!("Failed to render label '{}': {:?}", label, e);
        }
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

    // Update animation states smoothly using thread-local storage.
    CURRENT_SCALE.with(|scale_cell| {
        CURRENT_Y_OFFSET.with(|offset_cell| {
            let mut scales = scale_cell.borrow_mut();
            let mut offsets = offset_cell.borrow_mut();

            // Ensure vectors are properly sized.
            if scales.len() < buttons.len() {
                scales.resize(buttons.len(), 1.0);
            }
            if offsets.len() < buttons.len() {
                offsets.resize(buttons.len(), 0.0);
            }

            for (i, _button) in buttons.iter().enumerate() {
                let target_scale = if i as i32 == hover_button {
                    HOVER_SCALE
                } else {
                    1.0
                };

                if scales[i] < target_scale {
                    scales[i] += ANIMATION_SPEED;
                    if scales[i] > target_scale {
                        scales[i] = target_scale;
                    }
                } else if scales[i] > target_scale {
                    scales[i] -= ANIMATION_SPEED;
                    if scales[i] < target_scale {
                        scales[i] = target_scale;
                    }
                }

                // Animate radial expansion
                let target_y = if i as i32 == hover_button {
                    HOVER_Y_OFFSET
                } else {
                    0.0
                };
                if (offsets[i] - target_y).abs() > 0.1 {
                    offsets[i] += (target_y - offsets[i]) * 0.05;
                } else {
                    offsets[i] = target_y;
                }
            }
        })
    });

    // Draw each button wedge
    for (i, button) in buttons.iter().enumerate() {
        let button_start = start_angle + (i as f64 * wedge_size);
        let button_end = button_start + wedge_size;
        let is_hover = i as i32 == hover_button;

        let scale = CURRENT_SCALE.with(|cell| cell.borrow().get(i).copied().unwrap_or(1.0));

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
            button.icon_char,
            button.show_label,
        );
    }

    // Inner circle is now just empty space (no outline or fill)
}
