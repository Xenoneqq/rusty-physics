use macroquad::prelude::*;
use macroquad::rand::{gen_range};

const GRAVITY: f32 = 500.0;
const BOUNCE_DAMPING: f32 = 0.7;
const PARTICLE_RADIUS: f32 = 5.0;
const DRAG_FORCE: f32 = 100.0;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    lifetime: f32
}

impl Particle {
    fn update(&mut self, dt: f32) {
        // DECREASING TIME
        self.lifetime -= dt;

        // GRAVITY
        self.vel.y += GRAVITY * dt;
        self.pos += self.vel * dt;

        // DETECTING IMPACT WITH GROUND
        if self.pos.y + PARTICLE_RADIUS > screen_height() {
            // BOUNCES
            self.pos.y = screen_height() - PARTICLE_RADIUS;
            self.vel.y = -self.vel.y * BOUNCE_DAMPING;

            // GROUND DRAG
            let drag = DRAG_FORCE * dt;
            if self.vel.x != 0.0 {
                if (self.vel.x - drag).abs() > drag {
                    if self.vel.x > 0.0 {
                        self.vel.x = self.vel.x - drag;
                    }
                    else
                    {
                        self.vel.x = self.vel.x + drag;
                    }
                }
                else{
                    self.vel.x = 0.0;
                }
            }
        }
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, PARTICLE_RADIUS, YELLOW);
    }

    fn alive(&self) -> bool {
        return self.lifetime > 0.0;
    }
}

#[macroquad::main("Particle Explosion")]
async fn main() {
    let mut particles: Vec<Particle> = vec![];

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
            for _ in 0..20 {
                let angle = gen_range(0.0, std::f32::consts::TAU);
                let speed = gen_range(100.0, 300.0);
                let velocity = vec2(angle.cos(), angle.sin()) * speed;
                let life = gen_range(6.0 , 12.0);
                particles.push(Particle {
                    pos: center,
                    vel: velocity,
                    lifetime: life,
                });
            }
        }

        let dt = get_frame_time();

        for p in particles.iter_mut() {
            p.update(dt);
            p.draw();
        }

        particles.retain(|p| p.alive());

        draw_text("Left Click to EXPLODE PARTICLES!", 20.0, 20.0, 20.0, WHITE);
        let count_text = format!("Particles : {}", particles.len());
        draw_text(&count_text, 20.0, 40.0, 20.0, WHITE);
        next_frame().await;
    }
}
