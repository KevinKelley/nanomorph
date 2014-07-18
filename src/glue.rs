
#![macro_escape]

use std::cell::Cell; // for glfw error count
use glfw;
//use gl;


#[macro_export]
macro_rules! glcheck {
    ($e: expr) => ({
        let result = $e;
        let error_code = gl::GetError();
        if error_code != 0 {
            let description = match error_code {
                gl::INVALID_ENUM =>                  "GL_INVALID_ENUM",
                gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
                gl::INVALID_OPERATION =>             "GL_INVALID_OPERATION",
                gl::INVALID_VALUE =>                 "GL_INVALID_VALUE",
                gl::NO_ERROR =>                      "GL_NO_ERROR",
                gl::OUT_OF_MEMORY =>                 "GL_OUT_OF_MEMORY",
                _ => fail!("Bad gl error code: {}", error_code),
            };
            fail!("gl error: {}({})", description, error_code);
        }
        result
    })
}

/// give GLFW a way to report errors, and count them.
pub fn error_callback(_: glfw::Error, description: String, error_count: &Cell<uint>) {
    println!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}

