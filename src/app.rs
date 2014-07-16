

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

pub struct App {
	pub glfw: Glfw;
	//pub gl  : ;
	pub vg  : nanovg::Ctx;
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
        init_gl();

        let vg: nanovg::Ctx = nanovg::Ctx::create_gL3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);
        assert!(!vg.ptr.is_null());


        glfw.set_swap_interval(0);

        glfw.set_time(0.0);
        let mut prevt = glfw.get_time();

        App {
        	glfw: glfw,
        	gl: gl,
        	vg: vg
        }
	}

	fn main(&mut self) {
	}
}