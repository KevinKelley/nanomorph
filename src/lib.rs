#![crate_name = "nanomorph"]
#![desc = "a tiny morphic implementation on nanoVG"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(globs)]
#![feature(macro_rules)]
#![allow(non_snake_case_functions)]
#![allow(dead_code)]
#![allow(unused_variable)]

extern crate libc;
extern crate native;
extern crate glfw;
extern crate gl;
extern crate nanovg;

//pub use gfx::*;
//    use rawlink::*;

    mod rawlink;
pub mod glue; // glcheck macro
pub mod app;
pub mod window;
pub mod widget;
pub mod draw;
pub mod gfx;
pub mod icons;