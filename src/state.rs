use std::borrow::Borrow;
use std::ptr;
use glm::Vec2;
use sdl2::event::Event;
use crate::game::Game;
use crate::render::buffer::{BufferObject, BufferType, BufferUsage};
use crate::render::GlDataType;
use crate::render::shader::{ComputeProgram, Program, Shader};
use crate::render::texture::Texture;
use crate::render::vertex_arrays::VertexArrayObject;


pub struct GameState {
    pub(crate) screen_vao: VertexArrayObject,
    pub(crate) screen_program: Program,
}

impl GameState {
    pub fn new(game: &mut Game) -> Self {
        let window_size = game.window_size();

        let mut screen_program = {
            let vert = Shader::from_vertex_source(include_str!("shaders/screen.vert")).unwrap();
            let frag = Shader::from_frag_source(include_str!("shaders/screen.frag")).unwrap();
            Program::from_shaders(&[vert, frag]).unwrap()
        };
        screen_program.set_used();
        screen_program.set_vec2("windowSize", Vec2::new(window_size.0 as f32, window_size.1 as f32));

        let screen_vao = {
            #[repr(C, packed)]
            struct Vertex {
                pos: Vec2,
                uv: Vec2,
            }
            const SCREEN_VERTICES: [Vertex; 4] = [
                Vertex {
                    pos: Vec2::new(-1., -1.),
                    uv: Vec2::new(0.0, 1.0),
                },
                Vertex {
                    pos: Vec2::new(1., -1.),
                    uv: Vec2::new(1.0, 1.0),
                },
                Vertex {
                    pos: Vec2::new(1., 1.),
                    uv: Vec2::new(1.0, 0.0),
                },
                Vertex {
                    pos: Vec2::new(-1., 1.),
                    uv: Vec2::new(0.0, 0.0),
                },
            ];

            const SCREEN_INDICES: [u32; 6] = [
                0, 1, 2,
                2, 3, 0
            ];

            let vbo = BufferObject::create_vbo(
                &SCREEN_VERTICES,
                BufferUsage::StaticDraw,
            );

            let index_buffer = BufferObject::with_data(
                BufferType::ElementArray,
                &SCREEN_INDICES,
                BufferUsage::StaticDraw,
            );

            // let mut index_buffer = BufferObject::gen(1, BufferType::ElementArray);
            // index_buffer.set_data(&SCREEN_INDICES, BufferUsage::StaticDraw);

            VertexArrayObject::new_arrays(&vbo, Some(&index_buffer), |a| {
                a.vector(GlDataType::Float, 2); // position
                a.vector(GlDataType::Float, 2); // Tex Coord
            })
        };

        Self {
            screen_vao,
            screen_program,
        }
    }

    pub fn handle_event(&mut self, event: Event) -> bool {
        false
    }

    pub fn update(&mut self, game: &mut Game, delta: f64) {}

    pub fn render(&mut self, game: &mut Game, delta: f64) {
        unsafe {
            self.screen_program.set_used();
            self.screen_vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}