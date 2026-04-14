use macroquad::prelude::*;

const POINT_COUNT: usize = 250;
const LINE_DISTANCE: f32 = 120.0;

struct Point {
    pos: Vec2,
    vel: Vec2,
}

#[macroquad::main("Spider Cursor")]
async fn main() {
    let mut points: Vec<Point> = (0..POINT_COUNT)
        .map(|_| Point {
            pos: vec2(
                rand::gen_range(0.0, screen_width()),
                rand::gen_range(0.0, screen_height()),
            ),
            vel: vec2(
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
            ),
        })
        .collect();

    loop {
        clear_background(BLACK);

        let (mx, my) = mouse_position();
        let mouse = vec2(mx, my);

        // Update + draw points
        for p in &mut points {
            p.pos += p.vel;

            // Bounce
            if p.pos.x < 0.0 || p.pos.x > screen_width() {
                p.vel.x *= -1.0;
            }
            if p.pos.y < 0.0 || p.pos.y > screen_height() {
                p.vel.y *= -1.0;
            }

            draw_circle(p.pos.x, p.pos.y, 1.5, WHITE);

            // Draw lines to cursor
            let dist = p.pos.distance(mouse);

            if dist < LINE_DISTANCE {
                let alpha = 1.0 - (dist / LINE_DISTANCE);

                draw_line(
                    mouse.x,
                    mouse.y,
                    p.pos.x,
                    p.pos.y,
                    1.0,
                    Color::new(1.0, 1.0, 1.0, alpha),
                );
            }
        }

        // Draw spider body
        draw_circle(mouse.x, mouse.y, 6.0, ORANGE);

        // Legs (simple version)
        for i in 0..8 {
            let angle = i as f32 / 8.0 * std::f32::consts::TAU;
            let dir = vec2(angle.cos(), angle.sin());

            draw_line(
                mouse.x,
                mouse.y,
                mouse.x + dir.x * 20.0,
                mouse.y + dir.y * 20.0,
                2.0,
                GRAY,
            );
        }

        // Eyes
        draw_circle(mouse.x - 2.0, mouse.y - 2.0, 1.5, RED);
        draw_circle(mouse.x + 2.0, mouse.y - 2.0, 1.5, RED);

        next_frame().await;
    }
}