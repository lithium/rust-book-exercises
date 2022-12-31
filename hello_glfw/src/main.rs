extern crate glfw;
use glfw::{Action, Context, Key};

extern crate gl;
use gl::types::*;

use std::str;
use std::ptr;
use std::ffi::CString;

pub mod ogl;
use crate::ogl::ogl::Shader;


fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello glfw", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    
    let vertex_shader = Shader::from_source(&include_str!("shaders/triangle.vert"), gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::from_source(&include_str!("shaders/triangle.frag"), gl::FRAGMENT_SHADER).unwrap();


    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.5, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe { gl::Viewport(0, 0, width, height) }
        },
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        _ => {}
    }
}


