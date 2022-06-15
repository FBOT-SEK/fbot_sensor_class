use ev3dev_lang_rust::sensors::{SensorPort, ColorSensor};

use std::thread::{spawn};
use std::sync::{Arc, Mutex};

use std::error::Error;

use csv::{WriterBuilder, ReaderBuilder};
use serde::Serialize;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ColorTrain {
    Red,
    Blue,
    Yellow,
    White,
    Black,
    None,
}

impl FromStr for ColorTrain {
    type Err = ();

    fn from_str(input: &str) -> Result<ColorTrain, Self::Err> {
        match input {
            "Red" => Ok(ColorTrain::Red),
            "Blue" => Ok(ColorTrain::Blue),
            "Yellow" => Ok(ColorTrain::Yellow),
            "White" => Ok(ColorTrain::White),
            "Black" => Ok(ColorTrain::Black),
            _ => Err(()),
        }
    }
}

type RGB = (u8, u8, u8);
type HSV = (usize, usize, usize);
pub type ColorMap = Vec<Vec<ColorTrain>>;

const SENSOR_ARRAY_SIZE: usize = 10;
pub const H_SIZE: usize = 360;
pub const V_SIZE: usize = 100;

#[derive(Debug, Clone)]
pub struct FbotColorSensor {
    sensor: ColorSensor,
    value: Arc<Mutex<[Option<RGB>; SENSOR_ARRAY_SIZE]>>,
    color_map: ColorMap,
}

impl FbotColorSensor {
    pub fn new(sensor_port: SensorPort) -> Self {
        FbotColorSensor {
            value: Arc::new(Mutex::new([None; SENSOR_ARRAY_SIZE])),
            sensor: ColorSensor::get(sensor_port).unwrap(),
            color_map: vec![vec![ColorTrain::None; V_SIZE]; H_SIZE],
        }        
    }

    pub fn set_color_map(&mut self, color_map: ColorMap) {
        self.color_map = color_map;
    }

    pub fn get_color_map(&self) -> ColorMap {
        self.color_map.clone()
    }

    pub fn update_value(&self) {
        let value = self.value.clone();

        self.sensor.set_mode_rgb_raw().unwrap();
        let sensor = self.sensor.clone();

        spawn(move || {
            loop {
                let mut locked_value = value.lock().unwrap();

                for i in 0..SENSOR_ARRAY_SIZE-1 {
                    locked_value[i] = locked_value[i + 1];
                }

                let rgb = sensor.get_rgb().unwrap();
                let rgb: RGB = (rgb.0 as u8, rgb.1 as u8, rgb.2 as u8);

                locked_value[SENSOR_ARRAY_SIZE-1] = Some(rgb);

                drop(locked_value);
            }
        });
    }

    pub fn get_raw_value(&self) -> [Option<RGB>; SENSOR_ARRAY_SIZE] {
        let locked_value = self.value.lock().unwrap();

        locked_value.clone()
    }

    pub fn get_avg_value(&self) -> RGB {
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

    pub fn rgb_to_hsv(rgb: RGB) -> HSV {
        let r = rgb.0 as f32 / 255.0;
        let g = rgb.1 as f32 / 255.0;
        let b = rgb.2 as f32 / 255.0;
    
        let max = f32::max(f32::max(r, g), b);
        let min = f32::min(f32::min(r, g), b);
    
        let h = if max == r && g >= b {
            60.0 * ((g - b) / (max - min))
        } else if max == r && g < b {
            60.0 * ((g - b) / (max - min)) + 360.0
        } else if max == g {
            60.0 * ((b - r) / (max - min)) + 120.0
        } else if max == b {
            60.0 * ((r - g) / (max - min)) + 240.0
        } else {
            0.0
        };
    
        let s = ((max - min) / max) * 100.0;
        let v = max * 100.0;
    
        (h as usize, s as usize, v as usize)
    }

    pub fn csv_write(color_map: ColorMap) -> Result<(), Box<dyn Error>> {
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_path("train.csv")?;
    
        //loop over color_map
        for i in 0..color_map.len() {
            wtr.serialize(&color_map[i])?;
        }
    
        wtr.flush()?;
        Ok(())
    }

    pub fn csv_read() -> Result<ColorMap, Box<dyn Error>> { 
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path("color_map.csv")?;
    
        let mut color_map = vec![vec![ColorTrain::None; V_SIZE]; H_SIZE];
    
        for (h, result) in rdr.records().enumerate() {
            let record = result?;
            
            let row: Vec<ColorTrain> = record.deserialize(None).unwrap();
    
            color_map[h] = row;
        }
    
        Ok(color_map)
    }
}