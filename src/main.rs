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
    lifetime: f32,

    ball_color: Color,
    shadow_color: Color
}

impl Particle {
    fn update(&mut self, dt: f32) {
        
        // ELIMINATING UNWANTED BEHAVIOUR
        if dt < 0.0 {
            return;
        }
        
        // DECREASING TIME
        self.lifetime -= dt;
        
        if !self.is_moving(){
            return;
        }

        // GRAVITY
        if self.verticel_pos > 1.0 {
            self.vertical_vel -= GRAVITY * dt;
        }
        else if self.vertical_vel.abs() < 10.0 {
            self.vertical_vel = 0.0;
            self.verticel_pos = 0.0;
        }
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

            // GROUND DRAG Y
            let drag = DRAG_FORCE * dt;
            if self.vel.x != 0.0 {
                let sign = self.vel.x.signum();
                let new_v = self.vel.x - sign * drag;

                self.vel.x = if new_v.signum() != sign { 0.0 } else { new_v };
            }

            // GROUND DRAG X
            if self.vel.y != 0.0 {
                let sign = self.vel.y.signum();
                let new_v = self.vel.y - sign * drag;

                self.vel.y = if new_v.signum() != sign { 0.0 } else { new_v };
            }

        }
    }

    fn draw(&mut self) {
        self.ball_color.a = (self.lifetime / 5.0).min(1.0);
        draw_circle(self.pos.x, self.pos.y - self.verticel_pos, PARTICLE_RADIUS, self.ball_color);
    }

    fn draw_shadow(&mut self) {
        self.shadow_color.a = (self.lifetime / 5.0).min(1.0);
        draw_circle(self.pos.x, self.pos.y + SHADOW_OFFSET, PARTICLE_RADIUS * (1.0 - (self.verticel_pos / 300.0).min(1.0)), self.shadow_color);
    }

    fn alive(&self) -> bool {
        return self.lifetime > 0.0;
    }

    fn is_moving(&self) -> bool {
        self.vel.x != 0.0 || self.vel.y != 0.0 || self.vertical_vel != 0.0 || self.verticel_pos != 0.0
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

fn calculate_correct_frametime(vec: &mut Vec<f32>, dt: f32) -> f32 {
    vec.push(dt);

    while vec.len() > 11 {
        vec.remove(0);
    }

    let mut sorted = vec.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if dt > sorted[vec.len() / 2] * 2.0 {
        return sorted[vec.len() / 2];
    }
    dt
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut particles: Vec<Particle> = vec![];
    let mut frametimes: Vec<f32> = Vec::new();

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
                    ball_color: Color::new(0.376, 0.168, 1.0, 1.0),
                    shadow_color: Color::new(0.184, 0.109, 0.321, 1.0),
                });
            }
        }

        let dt = calculate_correct_frametime(&mut frametimes, get_frame_time());

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
        let fps_text = format!("FPS: {:.1}", 1.0 / dt);
        draw_text(&fps_text, 20.0, 60.0, 20.0, WHITE);
        next_frame().await;
    }
}