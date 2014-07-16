
#![feature(globs)]
#![feature(macro_rules)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variable)]

extern crate num;
extern crate native;
extern crate libc;
extern crate glfw;
extern crate gl;
extern crate nanovg;

use glfw::Context;
use nanovg::Ctx;



mod perf;
mod demo;


struct DemoApp {
    mut blowup: bool = false;
    mut screenshot: bool = false;
    mut premult: bool = false;
}

impl App for DemoApp {
    fn main()
    {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        // set up GLFW error callback, with an error-counter
        glfw.set_error_callback(Some(
            glfw::Callback {
                f: error_callback,
                data: Cell::new(0),
            }
        ));


        glfw.window_hint(glfw::ContextVersion(3, 2));
        glfw.window_hint(glfw::OpenglForwardCompat(true));
        glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
        glfw.window_hint(glfw::OpenglDebugContext(true));

        let (window, events) = glfw.create_window(1100, 800, "NanoVG GL3 example", glfw::Windowed)
            .expect("Failed to create GLFW window.");

        // window.set_key_callback(key);
        window.set_key_polling(true);

        window.make_current();


        // use glfw to load GL function pointers
        glcheck!(gl::load_with(|name| glfw.get_proc_address(name)));
        init_gl();

        let vg: nanovg::Ctx = nanovg::Ctx::create_gL3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);
        assert!(!vg.ptr.is_null());

        let mut data = demo::DemoData::load(&vg);


        glfw.set_swap_interval(0);

        glfw.set_time(0.0);
        let mut prevt = glfw.get_time();

        let mut fps = perf::PerfGraph::init(perf::FPS, "Frame Time");

        while !window.should_close()
        {
            let premult = unsafe { premult };
            let blowup  = unsafe { blowup  };

            let t: f64 = glfw.get_time();
            let dt: f64 = t - prevt;
            prevt = t;
            fps.update(dt);

            let (mx, my) = window.get_cursor_pos(); // (f64,f64)
            let (winWidth, winHeight) = window.get_size();  // (i32,i32)
            let (fbWidth, fbHeight) = window.get_framebuffer_size();
            // Calculate pixel ration for hi-dpi devices.
            let pxRatio = fbWidth as f32 / winWidth as f32;

            // Update and render
            glcheck!(gl::Viewport(0, 0, fbWidth, fbHeight));
            if premult {
                glcheck!(gl::ClearColor(0.0, 0.0, 0.0, 0.0));
            } else {
                glcheck!(gl::ClearColor(0.3, 0.3, 0.32, 1.0));
            }
            glcheck!(gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT));

            glcheck!(gl::Enable(gl::BLEND));
            glcheck!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
            glcheck!(gl::Enable(gl::CULL_FACE));
            glcheck!(gl::Disable(gl::DEPTH_TEST));


            vg.begin_frame(winWidth, winHeight, pxRatio as f32);

            demo::render_demo(&vg, mx as f32,my as f32, winWidth as f32,winHeight as f32, t as f32, blowup, &data);
            fps.render(&vg, 5.0, 5.0);

            vg.end_frame();


            gl::Enable(gl::DEPTH_TEST);

            unsafe {
                if screenshot {
                    screenshot = false;
                    demo::save_screenshot(fbWidth as u32, fbHeight as u32, premult, "dump.png");
                }
            }

            window.swap_buffers();

            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&window, event);
            }
        }

    // cleanup should be handled by Drop trait
    //  freeDemoData(vg, &data);
    //  nvgDeleteGLES3(vg);
    //  glfwTerminate();
    }

}
