use core::panic;
use std::{mem::size_of, os::raw::c_void};

use bytemuck;
use glad_gl::gl::{self, GLfloat, GLsizei, GLuint};
use glfw::{Action, Context, Key};

const VERTEX_SRC: &str = include_str!("vertex.glsl");
const FRAG_SRC: &str = include_str!("frag.glsl");

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(
            800,
            600,
            "[glad] Rust - OpenGL with GLFW",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    //vertices
    let vertices: &[GLfloat] = &[
        0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
        0.0, 0.8660254037844386, 0.0, 0.0, 1.0, 0.0, 
        -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 
    ];

    let indices: &[GLuint] = &[0, 1, 2];

    // load gl functions
    gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    // load shader program
    let shader_program = unsafe {
        let vertex_shader = gl::CreateShader(gl::enumerations::VERTEX_SHADER);
        let frag_shader = gl::CreateShader(gl::enumerations::FRAGMENT_SHADER);

        let mut success_status = 0;
        //vertex
        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERTEX_SRC.as_ptr() as *const i8),
            &0 as *const i32,
        );
        gl::CompileShader(vertex_shader);

        let info_log = bytemuck::allocation::zeroed_slice_box::<i8>(512);

        gl::GetShaderiv(
            vertex_shader,
            gl::enumerations::COMPILE_STATUS,
            &mut success_status as *mut i32,
        );
        if success_status != 0 {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                0 as *mut GLsizei,
                Box::<[i8]>::into_raw(info_log) as *mut i8,
            );
            panic!("Failed to Load Vertex Shader!");
        }

        //fragment
        gl::ShaderSource(
            frag_shader,
            1,
            &(FRAG_SRC.as_ptr() as *const i8),
            &0 as *const i32,
        );
        gl::CompileShader(frag_shader);
        let mut success_status = 0;

        let info_log = bytemuck::allocation::zeroed_slice_box::<i8>(512);

        gl::GetShaderiv(
            vertex_shader,
            gl::enumerations::COMPILE_STATUS,
            &mut success_status as *mut i32,
        );
        if success_status != 0 {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                0 as *mut GLsizei,
                Box::<[i8]>::into_raw(info_log) as *mut i8,
            );
            panic!("Failed to Load Fragment Shader!");
        }

        // link shader programs
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, frag_shader);
        gl::LinkProgram(shader_program);
        gl::GetProgramiv(
            shader_program,
            gl::enumerations::LINK_STATUS,
            &mut success_status as *mut i32,
        );

        if success_status != 0 {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                0 as *mut GLsizei,
                Box::<[i8]>::into_raw(info_log) as *mut i8,
            );

            panic!("Failed to Load Shader Program!");
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(frag_shader);
        shader_program
    };

    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;
    let mut ebo: GLuint = 0;

    // generate buffers
    unsafe {
        gl::GenVertexArrays(1, &mut vao as *mut GLuint);
        gl::GenBuffers(1, &mut vbo as *mut GLuint);
        gl::GenBuffers(1, &mut ebo as *mut GLuint);
    }

    // upload data
    unsafe {
        gl::UseProgram(shader_program);
        gl::BindVertexArray(vao);

        //upload vertices
        gl::BindBuffer(gl::enumerations::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::enumerations::ARRAY_BUFFER,
            (size_of::<GLfloat>() * vertices.len()).try_into().unwrap(),
            vertices.as_ptr() as *const c_void,
            gl::enumerations::STATIC_DRAW,
        );

        //upload indices
        gl::BindBuffer(gl::enumerations::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::enumerations::ELEMENT_ARRAY_BUFFER,
            (size_of::<GLfloat>() * indices.len()).try_into().unwrap(),
            indices.as_ptr() as *const c_void,
            gl::enumerations::STATIC_DRAW,
        );
    }

    // describe the vertex data
    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::enumerations::ARRAY_BUFFER, vbo);
        // position description
        gl::VertexAttribPointer(
            0,
            3,
            gl::enumerations::FLOAT,
            gl::enumerations::FALSE,
            6 as gl::types::GLsizei * size_of::<GLfloat>() as gl::types::GLsizei,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);

        // color description
        gl::VertexAttribPointer(
            1,
            3,
            gl::enumerations::FLOAT,
            gl::enumerations::FALSE,
            6 as gl::types::GLsizei * size_of::<GLfloat>() as gl::types::GLsizei,
            (3 * size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        //unbind shit just in case
        gl::BindBuffer(gl::enumerations::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // clear stuff
        unsafe {
            gl::ClearColor(0.1, 0.2, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        //draw stuff
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::enumerations::TRIANGLES,
                3,
                gl::enumerations::UNSIGNED_INT,
                0 as *const c_void,
            );
        }

        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
