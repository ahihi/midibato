extern crate midibato;

use midibato::{Control, Midibato};

fn main() {
    let controls = vec![
        Control {
            device_id: 0,
            channel: 0,
            cc: 20,
            default_value: 0,
            color: [255, 0, 0]
        },
        Control {
            device_id: 0,
            channel: 0,
            cc: 21,
            default_value: 127,
            color: [0, 0, 255]
        }
    ];
    let resolution = [800, 480];
    
    let mb = Midibato::new(controls, resolution);
}
