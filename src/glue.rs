
use std::cell::Cell; // for glfw error count

/// evaluate the expression, then check for GL error.
macro_rules! glcheck(
    ($e: expr) => (
        {
            $e;
            assert_eq!(gl::GetError(), 0);
        }
    )
)

/// give GLFW a way to report errors, and count them.
fn error_callback(_: glfw::Error, description: String, error_count: &Cell<uint>) {
    println!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}

fn init_gl() {
    glcheck!(gl::FrontFace(gl::CCW));
    glcheck!(gl::Enable(gl::DEPTH_TEST));
    glcheck!(gl::Enable(gl::SCISSOR_TEST));
    glcheck!(gl::DepthFunc(gl::LEQUAL));
    glcheck!(gl::FrontFace(gl::CCW));
    glcheck!(gl::Enable(gl::CULL_FACE));
    glcheck!(gl::CullFace(gl::BACK));
}

