use core::f32;
use std::{
	fs::File,
	io::{BufRead, BufReader,},
	time::Instant
};


fn main() {
	let moment = Instant::now();
	let measurements:Vec<Station> = generate_measurements("dummy.txt");
	println!("{:?}",measurements);
	println!("elapsed: {:?}",moment.elapsed());
}

fn generate_measurements(path:&str) -> Vec<Station>{
	let file = File::open(path).unwrap();
	let buffer = BufReader::new(file);
	let mut measurements:Vec<Station> = vec![];

	for (_,line) in buffer.lines().enumerate(){
		let temp_line = line.unwrap();
		let values:Vec<&str> = temp_line.split(';').collect();
		let existing_station: Option<&mut Station> = measurements.iter_mut().find(|station| {
			station.name == values[0]
		});
		let name = values[0];
		let value = values[1].parse::<f32>().unwrap();

		match existing_station {
			Some(station) => {
				station.min(value);
				station.max(value);
				station.average(value);
			},
			None => {
				measurements.push(
					Station::new(
						String::from(name),
						value,
						(value,1),
						value
					)
				);
			}
		}
	}

	measurements
}

#[derive(Debug)]
struct Station{
	name:String,
	min:f32,
	mean:(f32,i32),
	max:f32
}

impl Station {
	fn new(name:String,min:f32,mean:(f32,i32),max:f32) -> Self{
		Station { name, min,mean,max}
	}

	fn min(&mut self,num:f32){
		self.min = self.min.min(num)
	}

	fn max(&mut self,num:f32){
		self.max = self.max.max(num)
	}	

	fn average(&mut self,num:f32){
		self.mean.1 += 1;
		self.mean.0 = (self.mean.0 + num) / (self.mean.1 as f32);
	}
}
