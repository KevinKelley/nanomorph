
use glfw;
use nanovg;
use glfw::Context;

struct Window {
    pub glfw_window: glfw::Window,
    pub vg  : nanovg::Ctx,
    premult: bool,
    blowup: bool,
    screenshot: bool
}

impl Window {
	pub fn new(glfw:glfw::Glfw, title: &str, w: u32, h: u32) {
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

    fn handle_window_event(&mut self, window: &glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
                window.set_should_close(true)
            }
            glfw::KeyEvent(glfw::KeySpace, _, glfw::Press, _) => {
                self.blowup = !self.blowup;
            }
            glfw::KeyEvent(glfw::KeyS, _, glfw::Press, _) => {
                self.screenshot = true;
            }
            glfw::KeyEvent(glfw::KeyP, _, glfw::Press, _) => {
                self.premult = !self.premult;
            }
            _ => {}
        }
    }
}

