use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ ColorSensor };

// Factory design with rust

pub struct SensorColor {
    sensor: ColorSensor,
    value: u8,
    // color_map: vec![],
}

// implementar resul no new
// thread lendo o valor
// aprender tokio
impl SensorColor {
    pub fn new() -> Self {
        let sensor = ColorSensor::find().unwrap();
        sensor.set_mode_rgb_raw().unwrap();

        Self { 
            sensor,
            value: 2,
            // color_map: vec![],
        }
    }

    pub fn sensor(&self) -> &ColorSensor {
        &self.sensor
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}


pub fn algo() {
    println!("algo");
}