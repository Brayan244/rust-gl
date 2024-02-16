use sdl2::event::Event;

use crate::{objects::create_program, winsdl::WinSdl};

use std::time::Instant;

mod objects;
mod winsdl;

fn main() {
    println!("Hello, world!");

    let mut winsdl = WinSdl::new(800, 600, "OpenGL").unwrap();

    let start: Instant = Instant::now();

    let program = create_program().unwrap();
    program.set();

    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, -0.5, 0.5, 0.5];

    let indices: Vec<u32> = vec![0, 1, 2];

    let vbo = objects::Vbo::gen();
    vbo.set(&vertices);

    let vao = objects::Vao::gen();
    vao.set();

    let ibo = objects::Ibo::gen();
    ibo.set(&indices);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(
                sin_wave_from_instant(start, 0.0),
                sin_wave_from_instant(start, 1.0),
                sin_wave_from_instant(start, 0.5),
                1.0,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }

        winsdl.window.gl_swap_window();
    }
}

fn sin_wave_from_instant(instant: Instant, seed: f32) -> f32 {
    (instant.elapsed().as_secs_f32() + seed).sin() * 0.5 + 0.5
}
