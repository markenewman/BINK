use macroquad::prelude::*;
use std::collections::HashMap;

const ROOM: f32 = 5.0; // half-width of the square room
const PLAYER_R: f32 = 0.35;
const EYE_H: f32 = 1.6;
const LOOK_SPEED: f32 = 0.005;
const WALK_SPEED: f32 = 3.0;

// (center x, center z, half width x, half width z)
const PILLARS: [(f32, f32, f32, f32); 2] = [(2.0, -2.0, 0.5, 0.5), (-2.5, 1.5, 0.6, 0.6)];

fn window_conf() -> Conf {
    Conf {
        window_title: "BINK".to_owned(),
        ..Default::default()
    }
}

fn push_out(p: Vec2, center: Vec2, half: Vec2, r: f32) -> Vec2 {
    let d = p - center;
    let closest = center + vec2(d.x.clamp(-half.x, half.x), d.y.clamp(-half.y, half.y));
    let diff = p - closest;
    let dist = diff.length();
    if dist < r {
        if dist > 0.0001 {
            p + diff / dist * (r - dist)
        } else {
            p + vec2(r, 0.0)
        }
    } else {
        p
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut pos = vec2(0.0, 3.5); // x, z
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;
    let mut last_touch: HashMap<u64, Vec2> = HashMap::new();
    let mut last_mouse: Option<Vec2> = None;

    loop {
        let dt = get_frame_time();
        let w = screen_width();
        let h = screen_height();
        let stick_center = vec2(150.0, h - 150.0);
        let stick_radius = 100.0;
        let mut stick = Vec2::ZERO;

        // ---- touch input ----
        let ts = touches();
        for t in &ts {
            if t.position.x < w * 0.4 {
                // left side: joystick
                if !matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                    stick = (t.position - stick_center) / stick_radius;
                    if stick.length() > 1.0 {
                        stick = stick.normalize();
                    }
                }
                last_touch.remove(&t.id);
            } else {
                // right side: look
                match t.phase {
                    TouchPhase::Started => {
                        last_touch.insert(t.id, t.position);
                    }
                    TouchPhase::Moved | TouchPhase::Stationary => {
                        if let Some(prev) = last_touch.get(&t.id).copied() {
                            let d = t.position - prev;
                            yaw += d.x * LOOK_SPEED;
                            pitch -= d.y * LOOK_SPEED;
                        }
                        last_touch.insert(t.id, t.position);
                    }
                    _ => {
                        last_touch.remove(&t.id);
                    }
                }
            }
        }

        // ---- mouse look (desktop testing) ----
        if ts.is_empty() {
            let (mx, my) = mouse_position();
            let m = vec2(mx, my);
            if is_mouse_button_down(MouseButton::Left) {
                if let Some(prev) = last_mouse {
                    let d = m - prev;
                    yaw += d.x * LOOK_SPEED;
                    pitch -= d.y * LOOK_SPEED;
                }
                last_mouse = Some(m);
            } else {
                last_mouse = None;
            }
        }
        pitch = pitch.clamp(-1.4, 1.4);

        // ---- keyboard (desktop testing) ----
        let mut kb = Vec2::ZERO; // x = right, y = forward
        if is_key_down(KeyCode::W) { kb.y += 1.0; }
        if is_key_down(KeyCode::S) { kb.y -= 1.0; }
        if is_key_down(KeyCode::D) { kb.x += 1.0; }
        if is_key_down(KeyCode::A) { kb.x -= 1.0; }

        // ---- movement ----
        let mut input = vec2(stick.x + kb.x, -stick.y + kb.y);
        if input.length() > 1.0 {
            input = input.normalize();
        }
        let forward = vec2(yaw.sin(), -yaw.cos());
        let right = vec2(yaw.cos(), yaw.sin());
        pos += (right * input.x + forward * input.y) * WALK_SPEED * dt;

        // ---- collision ----
        pos.x = pos.x.clamp(-ROOM + PLAYER_R, ROOM - PLAYER_R);
        pos.y = pos.y.clamp(-ROOM + PLAYER_R, ROOM - PLAYER_R);
        for (cx, cz, hx, hz) in PILLARS.iter() {
            pos = push_out(pos, vec2(*cx, *cz), vec2(*hx, *hz), PLAYER_R);
        }

        // ---- draw 3D ----
        clear_background(Color::new(0.02, 0.02, 0.03, 1.0));
        let eye = vec3(pos.x, EYE_H, pos.y);
        let look = vec3(
            yaw.sin() * pitch.cos(),
            pitch.sin(),
            -yaw.cos() * pitch.cos(),
        );
        set_camera(&Camera3D {
            position: eye,
            target: eye + look,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        let floor = Color::new(0.10, 0.09, 0.08, 1.0);
        draw_cube(vec3(0.0, -0.06, 0.0), vec3(10.4, 0.1, 10.4), None, floor);
        draw_grid(10, 1.0, Color::new(0.2, 0.2, 0.22, 1.0), Color::new(0.12, 0.12, 0.14, 1.0));
        // ceiling
        draw_cube(vec3(0.0, 3.05, 0.0), vec3(10.4, 0.1, 10.4), None, Color::new(0.06, 0.06, 0.07, 1.0));
        // walls (slightly different shades so you can tell them apart)
        draw_cube(vec3(0.0, 1.5, -5.1), vec3(10.4, 3.0, 0.2), None, Color::new(0.26, 0.22, 0.18, 1.0));
        draw_cube(vec3(0.0, 1.5, 5.1), vec3(10.4, 3.0, 0.2), None, Color::new(0.18, 0.18, 0.24, 1.0));
        draw_cube(vec3(5.1, 1.5, 0.0), vec3(0.2, 3.0, 10.4), None, Color::new(0.22, 0.26, 0.20, 1.0));
        draw_cube(vec3(-5.1, 1.5, 0.0), vec3(0.2, 3.0, 10.4), None, Color::new(0.28, 0.20, 0.20, 1.0));
        // pillars
        for (cx, cz, hx, hz) in PILLARS.iter() {
            draw_cube(
                vec3(*cx, 1.5, *cz),
                vec3(hx * 2.0, 3.0, hz * 2.0),
                None,
                Color::new(0.45, 0.40, 0.32, 1.0),
            );
        }

        // ---- draw HUD ----
        set_default_camera();
        draw_text("BINK", 24.0, 52.0, 48.0, Color::new(0.9, 0.8, 0.5, 1.0));
        draw_circle_lines(stick_center.x, stick_center.y, stick_radius, 3.0, Color::new(1.0, 1.0, 1.0, 0.3));
        let thumb = stick_center + stick * stick_radius;
        draw_circle(thumb.x, thumb.y, 40.0, Color::new(1.0, 1.0, 1.0, 0.25));

        next_frame().await;
    }
}
