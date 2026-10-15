use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "BINK".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(Color::new(0.02, 0.02, 0.03, 1.0));

        // Slowly orbit a camera around a lone cube in the dark.
        let t = get_time() as f32 * 0.4;
        set_camera(&Camera3D {
            position: vec3(5.0 * t.sin(), 2.0, 5.0 * t.cos()),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.5, 0.0),
            ..Default::default()
        });

        draw_grid(20, 1.0, Color::new(0.25, 0.25, 0.3, 1.0), Color::new(0.1, 0.1, 0.12, 1.0));
        draw_cube(vec3(0.0, 0.5, 0.0), vec3(1.0, 1.0, 1.0), None, Color::new(0.55, 0.5, 0.4, 1.0));
        draw_cube_wires(vec3(0.0, 0.5, 0.0), vec3(1.0, 1.0, 1.0), Color::new(0.9, 0.8, 0.5, 1.0));

        set_default_camera();
        draw_text("BINK", 24.0, 56.0, 56.0, Color::new(0.9, 0.8, 0.5, 1.0));
        draw_text("hello, dark world", 24.0, 90.0, 28.0, GRAY);

        next_frame().await;
    }
}
