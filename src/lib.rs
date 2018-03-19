#[macro_use] extern crate glium;
extern crate glium_pib;
extern crate portmidi;

pub mod touch;

use std::default::Default;
use std::io;
use std::rc::Rc;
use std::sync::Arc;

use glium::Surface;
use portmidi::PortMidiDeviceId;

use touch::{Touch, TouchKind};

#[derive(Copy, Clone, Debug)]
pub struct Control {
    pub device_id: PortMidiDeviceId,
    pub channel: u8,
    pub cc: u8,
    pub default_value: u8,
    pub color: [f32; 3]
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2]
}
implement_vertex!(Vertex, position);

impl Vertex {
    fn new(x: u32, y: u32, resolution: [u32; 2]) -> Self {
        let x_max = (resolution[0] - 1) as f32;
        let y_max = (resolution[1] - 1) as f32;

        let x1 = (x as f32) / x_max * 2.0 - 1.0;
        let y1 = (y as f32) / y_max * 2.0 - 1.0;
        
        Vertex { position: [x1, y1] }
    }
}

#[derive(Debug)]
struct Area {
    upper_left: [u32; 2],
    lower_right: [u32; 2],
    control: Control,
    value: u8
}

impl Area {
    pub fn dimensions(&self) -> [u32; 2] {
        [self.lower_right[0] - self.upper_left[0] + 1, self.lower_right[1] - self.upper_left[1] + 1]
    }
    
    pub fn point_inside(&self, point: [u32; 2]) -> Option<[u32; 2]> {
        let is_inside =
            self.upper_left[0] <= point[0] && point[0] <= self.lower_right[0] &&
            self.upper_left[1] <= point[1] && point[1] <= self.lower_right[1];

        if is_inside {
            Some([point[0] - self.upper_left[0], point[1] - self.upper_left[1]])
        } else {
            None
        }
    }

    pub fn vertices(&self, resolution: [u32; 2]) -> Vec<Vertex> {
        let x0 = self.upper_left[0];
        let y0 = self.upper_left[1];
        let x1 = self.lower_right[0];
        let y1 = self.lower_right[1];
        vec![
            Vertex::new(x0, y0, resolution),
            Vertex::new(x0, y1, resolution),
            Vertex::new(x1, y1, resolution),
            Vertex::new(x1, y0, resolution)
        ]
    }
}

pub struct Midibato {
    resolution: [u32; 2],
    areas: Vec<Area>,
    facade: Rc<glium::backend::Context>,
    area_buffers: Vec<glium::VertexBuffer<Vertex>>,
    indices: glium::index::NoIndices,
    program: glium::Program
}

impl Midibato {
    pub fn new(controls: Vec<Control>, resolution: [u32; 2]) -> Self {
        let facade: Rc<glium::backend::Context> = {
            let system = glium_pib::System::new(Default::default());
            let system = match system {
                Ok(s) => s,
                Err(_) => {
                    panic!("Failed to use broadcom libraries.");
                }
            };
            let system = Arc::new(system);
            let facade = glium_pib::create_window_facade(
                &system,
                &std::default::Default::default()
            );
            match facade {
                Ok(f) => f,
                Err(_) => {
                    panic!("Failed to use broadcom libraries.");
                },
            }
        };

        let (width, height) = facade.get_framebuffer_dimensions();
        let control_count = controls.len();
        
        let width_div = width / (control_count as u32);
        let width_mod = width % (control_count as u32);

        let mut areas = Vec::new();
        let mut x_offset = 0;

        for control in controls {
            areas.push(Area {
                upper_left: [x_offset, 0],
                lower_right: [x_offset + width_div, height - 1],
                control: control,
                value: control.default_value
            });
            x_offset += width_div;
        }

        
        let area_buffers: Vec<glium::VertexBuffer<Vertex>> = areas.iter()
            .map(|area|
                 glium::VertexBuffer::new(&facade, &area.vertices([width, height]))
                 .expect("Failed to create vertex buffer")
            )
            .collect();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

        let vertex_shader_src = include_str!("glsl/slider.vert");
        let fragment_shader_src = include_str!("glsl/slider.frag");
        let program = glium::Program::from_source(
            &facade,
            vertex_shader_src,
            fragment_shader_src,
            None
        ).expect("Failed to create program");
        
        Midibato {
            resolution: resolution,
            areas: areas,
            facade: facade,
            area_buffers: area_buffers,
            indices: indices,
            program: program
        }
    }

    fn render(&self) {
        let mut target = glium::Frame::new(
            self.facade.clone(),
            self.facade.get_framebuffer_dimensions()
        );
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for (area, vertex_buffer) in self.areas.iter().zip(self.area_buffers.iter()) {
            println!("{:?} {:?}", area.control.color, area.value as f32);
            target.draw(
                vertex_buffer,
                &self.indices,
                &self.program,
                &uniform! {
                    //u_color: area.control.color,
                    u_red: [area.control.color[0]],
                    u_green: [area.control.color[1]],
                    u_blue: [area.control.color[2]],
                    u_value: [area.value as f32]
                },
                &Default::default()
            ).expect("Failed to draw");
        }
        
        target.finish().expect("Failed to finish");
    }

    pub fn run(&mut self) {
        self.render();
        
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read from stdin");

            if let Some(touch) = Touch::from_str(input.trim()) {
                if touch.kind != TouchKind::Release {
                    for area in self.areas.iter_mut() {
                        if let Some(point) = area.point_inside(touch.position) {
                            let value = (127 - 127 * point[1] / area.dimensions()[1]) as u8;

                            area.value = value;
                            println!("value: {:?}", value);
                        }
                    }

                    self.render();
                }
            }
        }
    }
}
