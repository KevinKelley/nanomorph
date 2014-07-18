

use std::cell::Cell; // for glfw error count
use native;
use glfw;
use gl;
use nanovg;
use glue::{error_callback};


#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}
fn main() {
    let mut app = App::new();
    app.main()
}


pub struct App {
	pub glfw: glfw::Glfw,
	pub vg  : nanovg::Ctx
}

impl App {
	fn new() -> App {
		let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        // set up GLFW error callback, with an error-counter
        glfw.set_error_callback(Some(
            glfw::Callback {
                f: error_callback,
                data: Cell::new(0),
            }
        ));


		// use glfw to load GL function pointers
        glcheck!(gl::load_with(|name| glfw.get_proc_address(name)));
        App::init_gl();

        let vg: nanovg::Ctx = nanovg::Ctx::create_gL3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);
        assert!(!vg.ptr.is_null());


        glfw.set_swap_interval(0);

        glfw.set_time(0.0);
        //let mut prevt = glfw.get_time();

        App {
        	glfw: glfw,
        	vg: vg
        }
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

	fn main(&mut self) {
	}
}