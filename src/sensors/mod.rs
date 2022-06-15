use ev3dev_lang_rust::sensors::{SensorPort};
use std::str::FromStr;

mod fbot_color_sensor;

pub fn new_color_sensor(sensor_port: SensorPort) -> fbot_color_sensor::FbotColorSensor {
    let mut color_sensor = fbot_color_sensor::FbotColorSensor::new(sensor_port);

    let color_map = match fbot_color_sensor::FbotColorSensor::csv_read() {
        Ok(map) => map,
        Err(_) =>  vec![vec![fbot_color_sensor::ColorTrain::None; fbot_color_sensor::V_SIZE]; fbot_color_sensor::H_SIZE]
    };

    color_sensor.set_color_map(color_map);
    color_sensor.update_value();
    color_sensor
}

pub fn train_color_sensor(sensor: &mut fbot_color_sensor::FbotColorSensor, color: &str, button: &ev3dev_lang_rust::Ev3Button) {
    let mut color_map = sensor.get_color_map();

    loop {
        let rgb = sensor.get_avg_value();
        let (h, _, v) = fbot_color_sensor::FbotColorSensor::rgb_to_hsv(rgb);
       
        color_map[h][v] = fbot_color_sensor::ColorTrain::from_str(color).unwrap();

        button.process();
        if button.is_left() {
            break;
        }
    }

    sensor.set_color_map(color_map.clone());
    fbot_color_sensor::FbotColorSensor::csv_write(color_map).unwrap();
}
