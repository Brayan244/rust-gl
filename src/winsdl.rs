use sdl2::{video::Window, EventPump, Sdl};

pub struct WinSdl {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: sdl2::video::GLContext,
    pub gl: (),
    pub event_pump: EventPump,
}

impl WinSdl {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        // set OpenGL version
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window("SDL2", width as u32, height as u32)
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        window
            .subsystem()
            .gl_set_swap_interval(sdl2::video::SwapInterval::VSync)
            .unwrap();

        let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

        Ok(WinSdl {
            sdl,
            window,
            gl_context,
            gl,
            event_pump,
        })
    }
}
