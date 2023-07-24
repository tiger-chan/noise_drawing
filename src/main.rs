use image::{ImageBuffer, RgbImage, Rgb, ImageFormat};
use ferro_noise::prelude::*;
use std::env;
use std::fs;
use chrono;

const SIZE: u32 = 1024;
const NOISE_SCALE: f32 = 0.0101;

fn main() {
	let args: Vec<String> = env::args().collect();
	let file_path = &args[1];
	println!("In file: {file_path}");

	let contents = fs::read_to_string(file_path);
	let result = match contents {
		Ok(data) => {
			let noise = ferro_noise::ser::f32::toml::from_str(&data);
			
			let img = match noise {
				Ok(mut noise) => {
					let noise = noise.get_mut("main").unwrap();
					let mut img: RgbImage =  ImageBuffer::new(SIZE, SIZE);
					for y in 0..SIZE {
						for x in 0..SIZE {
							let value = noise.sample_2d(x as f32 * NOISE_SCALE, y as f32 * NOISE_SCALE);
							let value = (value * 255.0) as u8;
							img.put_pixel(x, y, Rgb{0: [value, value, value]});
						}
					}
					Ok((img, data))
				}
				Err(x) => Err(x)
			};
			img
		},
		Err(x) => {
			println!("File not found");
			Err(x.to_string())
		},
	};

	let now = chrono::Utc::now().format("%Y%m%dT%H%M%S");
	let image_name = format!("./images/{}.png", now);
	let config_name = format!("./images/{}.toml", now);

	fs::create_dir_all("images").expect("Directory should have been created");

	match result {
		Ok((img, config)) => {
			img.save_with_format(&image_name, ImageFormat::Png).expect("Unable to write image file");
			fs::write(config_name, config).expect("Unable to write config file");
		}, 
		_ => {}
	}
}
