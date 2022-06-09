use std::thread::{sleep, spawn};
use std::sync::{Arc, Mutex};
use rand::Rng;
use std::env;

const SENSOR_ARRAY_SIZE: usize = 10;

type RGB = (u8, u8, u8);

#[derive(Debug)]
struct Sensor {
    value: Arc<Mutex<[Option<RGB>; SENSOR_ARRAY_SIZE]>>
}

impl Sensor {
    fn new() -> Self {
        Sensor {
            value: Arc::new(Mutex::new([None; SENSOR_ARRAY_SIZE]))
        }        
    }

    fn update_value(&self) {
        let value = self.value.clone();

        spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                let mut locked_value = value.lock().unwrap();
                let random_value = rng.gen_range(0..255);

                for i in 0..SENSOR_ARRAY_SIZE-1 {
                    locked_value[i] = locked_value[i + 1];
                }
                locked_value[SENSOR_ARRAY_SIZE-1] = Some((random_value, random_value, random_value));

                drop(locked_value);
            }
        });
    }

    fn get_raw_value(&self) -> [Option<RGB>; SENSOR_ARRAY_SIZE] {
        let locked_value = self.value.lock().unwrap();

        locked_value.clone()
    }

    fn get_avg_value(&self) -> RGB {
        let value = self.get_raw_value();
        
        let (sum_r, sum_g, sum_b) = value.iter().fold((0, 0, 0), |acc, v| {
            match v {
                Some(v) => (acc.0 + v.0 as u32, acc.1 + v.1 as u32, acc.2 + v.2 as u32),
                None => acc
            }
        });
        
        let avg_r =  (sum_r / SENSOR_ARRAY_SIZE as u32) as u8;
        let avg_g =  (sum_g / SENSOR_ARRAY_SIZE as u32) as u8;
        let avg_b =  (sum_b / SENSOR_ARRAY_SIZE as u32) as u8;
        
        (avg_r, avg_g, avg_b)
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    let sensor = Sensor::new();
    sensor.update_value();

    loop {
        println!("{:?}", sensor.get_avg_value());
        sleep(std::time::Duration::from_millis(100));
    }
}