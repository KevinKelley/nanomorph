

struct Window {

}

impl Window {
	pub fn new(title: &str, w: u32, h: u32) {
        glfw.window_hint(glfw::ContextVersion(3, 2));
        glfw.window_hint(glfw::OpenglForwardCompat(true));
        glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
        glfw.window_hint(glfw::OpenglDebugContext(true));

        let (window, events) = glfw.create_window(w, h, title, glfw::Windowed)
            .expect("Failed to create GLFW window.");

        // window.set_key_callback(key);
        window.set_key_polling(true);

        window.make_current();
	}

}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        glfw::KeyEvent(glfw::KeySpace, _, glfw::Press, _) => {
            unsafe {blowup = !blowup};
        }
        glfw::KeyEvent(glfw::KeyS, _, glfw::Press, _) => {
            unsafe {screenshot = true};
        }
        glfw::KeyEvent(glfw::KeyP, _, glfw::Press, _) => {
            unsafe {premult = !premult};
        }
        _ => {}
    }
}
