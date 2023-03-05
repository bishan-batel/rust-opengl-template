use std::borrow::Borrow;
use std::collections::HashSet;
use std::f32::consts;
use std::mem::size_of;
use std::ptr;
use std::time::Instant;
use gl::types::{GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use image::DynamicImage::ImageRgba32F;
use image::{EncodableLayout, GenericImageView, Rgba32FImage};
use image::imageops::FilterType;
use rand::thread_rng;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseWheelDirection;
use sdl2::sys::rand;
use sdl2::video::{GLContext, GLProfile, Window};

use crate::render::color::{ColorInternal, ColorRepr};
use crate::render::GlDataType;
use crate::render::shader::{ComputeProgram, Program, Shader};
use crate::render::texture::{ImageAccess, Texture, TextureTarget};
use crate::render::vertex_arrays::{AttributeLayout, VertexArrayObject};
use crate::render::buffer::{BufferObject, BufferType, BufferUsage};
use crate::state::{GameState};

const PARTICLE_COUNT: usize = 1000;

pub struct Game {
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    window: Window,
    event_pump: EventPump,
    gl_context: GLContext,
    running: bool,
    state: Option<GameState>,
    window_size: (i32, i32),
    keys_down: HashSet<Scancode>,
}


impl Game {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sdl = sdl2::init()?;

        let video_subsystem = sdl.video()?;

        let event_pump = sdl.event_pump()?;

        // set GL versions
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);

        gl_attr.set_context_version(4, 3);

        let display = video_subsystem.desktop_display_mode(0).unwrap();

        // create window
        let window = video_subsystem
            .window("Gaming", display.w as u32, display.h as u32)
            .fullscreen()
            .opengl()
            .build()?;

        // bind context
        let gl_context = window.gl_create_context()?;
        gl::load_with(|f| video_subsystem.gl_get_proc_address(f) as *const std::os::raw::c_void);

        Ok(Self {
            sdl,
            gl_context,
            video_subsystem,
            window_size: (window.size().0 as i32, window.size().1 as i32),
            window,
            event_pump,
            running: true,
            state: None,
            keys_down: HashSet::new(),
        })
    }

    pub fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {


            // handle event
            if let Some(state) = &mut self.state {
                if state.handle_event(event.clone()) {
                    continue;
                }
            }
            match event {
                Event::Quit { .. } => self.running = false,

                // Window Events
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => self.window_size = (width, height),
                    _ => {}
                },
                Event::KeyDown { scancode, .. } => if let Some(scancode) = scancode {
                    self.keys_down.insert(scancode);
                },
                Event::KeyUp { scancode, .. } => if let Some(scancode) = scancode {
                    self.keys_down.remove(&scancode);
                },
                _ => {}
            }
        }
    }

    pub unsafe fn init(&mut self) {
        gl::Viewport(0, 0, self.window_size.0 as i32, self.window_size.1 as i32);
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);

        self.state = Some(GameState::new(self));
    }

    pub fn update(&mut self, delta: f64) {
        if let Some(mut state) = self.state.take() {
            state.update(self, delta);
            self.state = Some(state);
        }
    }

    pub unsafe fn render(&mut self, mut delta: f64) {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        if let Some(mut state) = self.state.take() {
            state.render(self, delta);
            self.state = Some(state);
        }
        self.window.gl_swap_window();
    }

    #[inline]
    pub const fn is_running(&self) -> bool {
        self.running
    }

    pub fn resize_window(&mut self, size: (i32, i32)) {
        self.window_size = size;
        unsafe {
            gl::Viewport(0, 0, size.0, size.1);
        }
    }

    pub fn is_key_down(&self, key: Scancode) -> bool {
        self.keys_down.contains(&key)
    }
    pub const fn window_size(&self) -> (i32, i32) { self.window_size }
}