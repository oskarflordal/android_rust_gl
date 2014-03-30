//pub use r3d::landscape::*;
//pub use r3d::mesh::*;

#[feature(globs)];
#[feature(macro_rules)];
#[feature(default_type_params)];
#[allow(unused_imports)];
#[allow(unused_variable)];
#[allow(dead_code)];
#[allow(unreachable_code)];
#[allow(unused_unsafe)];
#[allow(unused_mut)];
#[allow(non_camel_case_types)];
#[macro_escape];


extern crate collections;
pub use std::str::raw::*;
pub use r3d::vecmath::*;
pub use std::vec;
pub use std::libc;
pub use std::libc::{c_int,c_char};
use shadertest::c_str;
use std::io;
use shadertest::*;
use r3d::gl::*;
use r3d::glut::*;


pub mod macros;	// must preceed others for macro visibility.
pub mod r3d;
pub mod shadertest;

// framework can be: Android, Glut, (iOS,..)

#[cfg(target_os = "android")]
extern { fn android_log_print(lvl:c_int,  s:*c_char);}

#[cfg(target_os = "android")]
fn log_print(level:int, s:&str) {
	unsafe {
		android_log_print(level as c_int, c_str(s));
	}
}

#[cfg(not(target_os = "android"))]
fn log_print(level:int, s:&str) {
	io::println(s);
}


static MAX_TOUCH_POINTERS:u32=12;

// TODO: These can be renamed away from android
// Its' really to be a superset of Android,iOS,Windows8,game-consoles,PC
struct AndroidInputSub {
	pointers:[((f32,f32,f32),u32),..12],
	accelerometer:(f32,f32,f32),
}

struct AndroidInput {
	// TODO: gamepad, keyboard... they do exist
	curr:AndroidInputSub,
	prev:AndroidInputSub
}
extern { fn android_get_inputs()->AndroidInput; }





// Desktop glut main loop, uses the app_create/display_create/render/display_destroy/destroy hooks; on android (iOS..) these same functions are just called by platform specific app loops
// It might be nice to make a rust trait object for all this, 
// however this is language independant. One can glue any other framework specifics ontop.

#[cfg(not(target_os = "android"))]
pub fn main()
{
	unsafe {
		let mut argc:c_int=0;
		glutInit((&mut argc) as *mut std::libc::c_int,0 as **libc::c_char );

		glutInitDisplayMode(GLUT_DEPTH | GLUT_DOUBLE | GLUT_RGBA);
		glutInitWindowSize(1280,800);
		let win=verify!(glutCreateWindow(c_str("Rust ShaderTest")) isnt 0);

		let mut app = app_create(0,0 as **std::libc::c_char,1280,800);
		app_display_create(app);
		glDrawBuffer(GL_BACK);
		glutReshapeWindow(1024,1024);
		glEnable(GL_DEPTH_TEST);

		while true {
			glutMainLoopEvent();
			app_render(app);
			glFlush();
			glutSwapBuffers();
		}
		app_display_destroy(app);
		app_destroy(app);
	}
}



