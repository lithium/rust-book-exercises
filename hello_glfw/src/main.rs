use glfw::{Action, Context, Key};
use gl::types::*;

mod ogl;
use crate::ogl::{Shader, Program};


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

    let shader_program = Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();
    shader_program.use_program();



    let vertices: [f32; 18] = [
        //positions         // colors
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0,        
         0.0,  0.5, 0.0,    0.0, 0.0, 1.0,
    ];

    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3, 
            gl::FLOAT, 
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as GLint,
            std::ptr::null()
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3, 
            gl::FLOAT, 
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as GLint,
            (3 * std::mem::size_of::<f32>()) as *const GLvoid, 
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    while !window.should_close() {
        shader_program.use_program();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
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


