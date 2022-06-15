extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{Ev3Button, Ev3Result};
use ev3dev_lang_rust::sensors::{SensorPort};

use std::env;

mod sensors;

fn main() -> Ev3Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let button = Ev3Button::new()?;
    let mut color_sensor = sensors::new_color_sensor(SensorPort::In1);


    sensors::train_color_sensor(&mut color_sensor, "Red", &button);
    println!("Treinado Red");
    std::thread::sleep(std::time::Duration::from_secs(5));
    sensors::train_color_sensor(&mut color_sensor, "Yellow", &button);
    println!("Treinado Yellow");

    // loop {
        // let avg_value = color_sensor.get_avg_value();

        // println!("{:?}", avg_value);
        

    //     button.process();
    //     if button.is_right() {
    //         break;
    //     }
    // };
    
    Ok(())
}