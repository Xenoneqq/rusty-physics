use macroquad::prelude::*;
use macroquad::rand::{gen_range};

const GRAVITY: f32 = 500.0;
const BOUNCE_DAMPING: f32 = 0.4;
const PARTICLE_RADIUS: f32 = 5.0;
const SHADOW_OFFSET: f32 = 3.0;
const DRAG_FORCE: f32 = 100.0;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    verticel_pos: f32,
    vertical_vel: f32,
    lifetime: f32
}

impl Particle {
    fn update(&mut self, dt: f32) {
        // DECREASING TIME
        self.lifetime -= dt;

        // GRAVITY
        self.vertical_vel -= GRAVITY * dt;
        self.verticel_pos += self.vertical_vel * dt;
        self.pos += self.vel * dt;

        // DETECTING IMPACT WITH GROUND
        if self.verticel_pos <= 0.0 {

            // PREVENT SINKING
            self.verticel_pos = 0.0;
            // BOUNCES
            if self.vertical_vel.abs() > 10.0{
                self.vertical_vel = -self.vertical_vel * BOUNCE_DAMPING;
            } else {
                self.vertical_vel = 0.0;
            }

            // GROUND DRAG
            let drag = DRAG_FORCE * dt;
            for v in [&mut self.vel.x, &mut self.vel.y] {
                if *v != 0.0 {
                    let sign = v.signum();
                    let new_v = *v - sign * drag;

                    if new_v.signum() != sign {
                        *v = 0.0;
                    } else {
                        *v = new_v;
                    }
                }
            }

        }
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y - self.verticel_pos, PARTICLE_RADIUS, YELLOW);
    }

    fn draw_shadow(&self) {
        draw_circle(self.pos.x, self.pos.y + SHADOW_OFFSET, PARTICLE_RADIUS, GRAY);
    }

    fn alive(&self) -> bool {
        return self.lifetime > 0.0;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle Explosion".to_owned(),
        window_width: 1000,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut particles: Vec<Particle> = vec![];

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let spawn_point = vec2(mouse_x, mouse_y);
            for _ in 0..20 {
                let angle = gen_range(0.0, std::f32::consts::TAU);
                let speed = gen_range(20.0, 60.0);
                let start_vertical_velocity = gen_range(100.0, 450.0);
                let velocity = vec2(angle.cos(), angle.sin()) * speed;
                let life = gen_range(6.0 , 12.0);
                particles.push(Particle {
                    pos: spawn_point,
                    vel: velocity,
                    verticel_pos: 0.01,
                    vertical_vel: start_vertical_velocity,
                    lifetime: life,
                });
            }
        }

        let dt = get_frame_time();

        for p in particles.iter_mut() {
            p.update(dt);
            p.draw_shadow();
        }

        for p in particles.iter_mut() {
            p.draw();
        }

        particles.retain(|p| p.alive());

        draw_text("Left Click to EXPLODE PARTICLES!", 20.0, 20.0, 20.0, WHITE);
        let count_text = format!("Particles : {}", particles.len());
        draw_text(&count_text, 20.0, 40.0, 20.0, WHITE);
        next_frame().await;
    }
}
