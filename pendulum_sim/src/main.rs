use macroquad::prelude::*;
use std::f32::consts::PI;

struct PendulumConfig {
    l1: f32,
    l2: f32,
    m1: f32,
    m2: f32,
    g: f32,
}

type State = [f32; 4];

fn derivatives(state: State, config: &PendulumConfig) -> State {
    let [t1, t2, w1, w2] = state;
    let delta = t1 - t2;

    let den1 = config.l1 * (2.0 * config.m1 + config.m2 - config.m2 * (2.0 * t1 - 2.0 * t2).cos());
    let num1 = -config.g * (2.0 * config.m1 + config.m2) * t1.sin()
        - config.m2 * config.g * (t1 - 2.0 * t2).sin()
        - 2.0 * delta.sin() * config.m2 * (w2 * w2 * config.l2 + w1 * w1 * config.l1 * delta.cos());
    let alpha1 = num1 / den1;

    let den2 = config.l2 * (2.0 * config.m1 + config.m2 - config.m2 * (2.0 * t1 - 2.0 * t2).cos());
    let num2 = 2.0 * delta.sin() * (
        w1 * w1 * config.l1 * (config.m1 + config.m2)
        + config.g * (config.m1 + config.m2) * t1.cos()
        + w2 * w2 * config.l2 * config.m2 * delta.cos()
    );
    let alpha2 = num2 / den2;

    [w1, w2, alpha1, alpha2]
}

fn rk4_step(state: State, dt: f32, config: &PendulumConfig) -> State {
    let k1 = derivatives(state, config);

    let mut s2 = [0.0; 4];
    for i in 0..4 { s2[i] = state[i] + 0.5 * dt * k1[i]; }
    let k2 = derivatives(s2, config);

    let mut s3 = [0.0; 4];
    for i in 0..4 { s3[i] = state[i] + 0.5 * dt * k2[i]; }
    let k3 = derivatives(s3, config);

    let mut s4 = [0.0; 4];
    for i in 0..4 { s4[i] = state[i] + dt * k3[i]; }
    let k4 = derivatives(s4, config);

    let mut next = [0.0; 4];
    for i in 0..4 {
        next[i] = state[i] + (dt / 6.0) * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
    }
    next
}

const INITIAL_STATE: State = [PI / 2.0, PI / 2.0, 0.0, 0.0];

#[macroquad::main("Interactive Double Pendulum")]
async fn main() {
    let mut config = PendulumConfig {
        l1: 160.0,
        l2: 160.0,
        m1: 10.0,
        m2: 10.0,
        g: 981.0,
    };

    let mut state: State = INITIAL_STATE;
    let dt = 0.016;
    let mut is_paused = false;
    
    let mut trail: Vec<Vec2> = Vec::new();
    let max_trail_len = 1500;

    loop {
        // --- KEYBOARD INPUT HANDLING ---

        // 1. Toggle Pause (Single Press)
        if is_key_pressed(KeyCode::Space) {
            is_paused = !is_paused;
        }

        // 2. Reset Simulation State (Single Press)
        if is_key_pressed(KeyCode::R) {
            state = INITIAL_STATE;
            trail.clear();
        }

        // 3. Clear Trajectory Trail (Single Press)
        if is_key_pressed(KeyCode::C) {
            trail.clear();
        }

        // 4. Adjust Gravity Continuously (Key Held Down)
        if is_key_down(KeyCode::Up) {
            config.g += 200.0 * get_frame_time();
        }
        if is_key_down(KeyCode::Down) {
            config.g = (config.g - 200.0 * get_frame_time()).max(0.0);
        }

        // --- PHYSICS UPDATE ---
        if !is_paused {
            for _ in 0..5 {
                state = rk4_step(state, dt / 5.0, &config);
            }

            let center = vec2(screen_width() / 2.0, screen_height() / 3.0);
            let p1 = center + vec2(config.l1 * state[0].sin(), config.l1 * state[0].cos());
            let p2 = p1 + vec2(config.l2 * state[1].sin(), config.l2 * state[1].cos());

            trail.push(p2);
            if trail.len() > max_trail_len {
                trail.remove(0);
            }
        }

        // --- RENDERING ---
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        let center = vec2(screen_width() / 2.0, screen_height() / 3.0);
        let p1 = center + vec2(config.l1 * state[0].sin(), config.l1 * state[0].cos());
        let p2 = p1 + vec2(config.l2 * state[1].sin(), config.l2 * state[1].cos());

        // Render trajectory path
        for i in 1..trail.len() {
            let alpha = i as f32 / trail.len() as f32;
            draw_line(
                trail[i - 1].x, trail[i - 1].y,
                trail[i].x, trail[i].y,
                2.0,
                Color::new(0.0, 0.9, 1.0, alpha * 0.7),
            );
        }

        // Render rods & bobs
        draw_line(center.x, center.y, p1.x, p1.y, 4.0, WHITE);
        draw_line(p1.x, p1.y, p2.x, p2.y, 4.0, WHITE);
        draw_circle(center.x, center.y, 6.0, GRAY);
        draw_circle(p1.x, p1.y, 12.0, RED);
        draw_circle(p2.x, p2.y, 12.0, GREEN);

        // --- ON-SCREEN HUD ---
        draw_text("CONTROLS:", 20.0, 30.0, 20.0, LIGHTGRAY);
        draw_text("[SPACE] Pause / Resume", 20.0, 55.0, 18.0, GRAY);
        draw_text("[R]     Reset Simulation", 20.0, 75.0, 18.0, GRAY);
        draw_text("[C]     Clear Trail", 20.0, 95.0, 18.0, GRAY);
        draw_text("[UP/DN] Adjust Gravity", 20.0, 115.0, 18.0, GRAY);

        let status_str = if is_paused { "PAUSED" } else { "RUNNING" };
        let status_color = if is_paused { YELLOW } else { GREEN };
        draw_text(&format!("State: {}", status_str), 20.0, 150.0, 22.0, status_color);
        draw_text(&format!("Gravity: {:.1}", config.g / 100.0), 20.0, 175.0, 22.0, WHITE);

        next_frame().await;
    }
}
