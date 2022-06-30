use gl::COLOR_BUFFER_BIT;
use glfw::{Action, Context, Key};

extern crate gl;
extern crate glfw;

pub fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "LearnOpenGLWithRust", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    // The locations of OpenGL functions isn't known at compile-time (because actual implementation of OpenGL standard is in drivers),
    // so we need to load them at runtime.
    gl::load_with(|symbol| window.get_proc_address(symbol));

    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        // Input.
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // Rendering.
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(COLOR_BUFFER_BIT)
        }

        // Check and call events and swap buffers.
        glfw.poll_events();
        window.swap_buffers();
    }
}

// Based on glfw-rs example on crates site.
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        _ => {}
    }
}
