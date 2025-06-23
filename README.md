# 🦀 Rusty Physics – 3D in 2D Physics Simulation 🦀

Rusty Physics is a lightweight and fun simulation project written in Rust using the `macroquad` game engine. It features a pseudo-3D (3D in 2D) visual effect with simple, yet satisfying, physics interactions. The project simulates bouncing balls with realistic motion and a clean, delta-time-independent update loop.

> This project was developed as part of a university Rust programming class.

---

## Features

- **Interactive Input** – Click anywhere on the screen to spawn an explosion of colorful balls.
- **Physics-Driven Motion** – Balls fall under gravity and respond to collisions with damping.
- **Smooth Simulation** – No stutter, no delta time bugs — just buttery-smooth animation.
- **Simple Yet Expandable** – A solid base for learning physics simulations or prototyping.

---

## Possible Additions

- **Optimization Improvements** – Handling particles in batches and utilizing multithreading for better performance.

---

## Setup `Linux / WSL2`

1. **Install Rust and Cargo**

    If you haven't already, install the Rust toolchain using [rustup](https://rustup.rs/):

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. **Clone the Repository**

    ```sh
    git clone https://github.com/Xenoneqq/rusty-physics
    cd rusty-physics
    ```

3. **Build and Run the Project**

    ```sh
    cargo build
    cargo run
    ```

    The simulation window should open, allowing you to interact with the physics demo in real time.