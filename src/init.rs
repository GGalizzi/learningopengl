use gl as raw;
use sdl2::{self, video::GLProfile};

pub fn gl(video: &sdl2::VideoSubsystem) {
    unsafe {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_flags().debug().set();
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(GLProfile::Core);
        //println!("errors {:?}", raw::GetError());

        println!("{:?}", gl_attr.context_version());
    }
}

pub fn load_gl(video: &sdl2::VideoSubsystem) {
    raw::load_with(|s| {
        video.gl_get_proc_address(s) as *const _
    });

    unsafe {
        raw::Viewport(
            0,
            0,
            crate::WINDOW_WIDTH as i32,
            crate::WINDOW_HEIGHT as i32,
        );

        raw::Enable(gl::DEPTH_TEST);
        raw::Enable(gl::CULL_FACE);
        raw::FrontFace(gl::CW);
    }
}
