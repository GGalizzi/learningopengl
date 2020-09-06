use gl as raw;
use sdl2::{self, video::GLProfile};

pub fn gl(video: &sdl2::VideoSubsystem) {
    unsafe {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        raw::load_with(|s| {
            video.gl_get_proc_address(s) as *const _
        });
        println!("errors {:?}", raw::GetError());
        raw::Viewport(
            0,
            0,
            crate::WINDOW_WIDTH as i32,
            crate::WINDOW_HEIGHT as i32,
        );
    }
}
