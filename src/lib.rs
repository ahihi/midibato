#[macro_use] extern crate glium;
extern crate glium_pib;
extern crate portmidi;

use portmidi::PortMidiDeviceId;

#[derive(Copy, Clone, Debug)]
pub struct Control {
    pub device_id: PortMidiDeviceId,
    pub channel: u8,
    pub cc: u8,
    pub default_value: u8,
    pub color: [u8; 3]
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2]
}
implement_vertex!(Vertex, position);

#[derive(Debug)]
struct Area {
    upper_left: [usize; 2],
    lower_right: [usize; 2],
    control: Control,
    value: u8
}

impl Area {
    pub fn point_inside(&self, point: [usize; 2]) -> Option<[usize; 2]> {
        let is_inside =
            self.upper_left[0] <= point[0] && point[0] <= self.lower_right[0] &&
            self.upper_left[1] <= point[1] && point[1] <= self.lower_right[1];

        if is_inside {
            Some([point[0] - self.upper_left[0], point[1] - self.upper_left[1]])
        } else {
            None
        }
    }
}

pub struct Midibato {
    areas: Vec<Area>
}

impl Midibato {
    pub fn new(controls: Vec<Control>, resolution: [usize; 2]) -> Self {
        let width = resolution[0];
        let height = resolution[1];
        let control_count = controls.len();
        
        let width_div = width / control_count;
        let width_mod = width % control_count;

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

        println!("{:?}", areas);
        
        Midibato {
            areas: areas
        }
    }

    /*fn hit(&self, point: [usize; 2]) -> Option<(Area, [usize; 2])> {
        for area in self.areas.iter() {            
            if let Some(p) = area.point_inside(point) {
                return Some((area, p))
            }
        }
        None
    }*/
}
