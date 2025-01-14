use std::{fs,env};
use std::io::{Write,BufWriter};
use rand::prelude::*;
use clap::Parser;

fn main(){
	let args = Args::parse();
	generate_measurements(args.count,args.file.as_str());
}

fn generate_measurements(count:u32,file_path:&str){
	//TODO handle file not found error
	fs::write(file_path, "").unwrap();
	let file = fs::OpenOptions::new().append(true).open(file_path).unwrap();
	let mut writer = BufWriter::new(file);
	let stations = get_weather_stations();
	
	let mut rng = thread_rng();
	for _ in 0..count  {
		let num:f32 = rng.gen_range(-15.0..50.0);
		let m = format!(
			"{};{}\n",
			stations[rng.gen_range(0..stations.len()-1)],
			num.round());
		writer.write(m.as_bytes()).unwrap();		
	}
}

fn get_weather_stations() -> Vec<String> {
	let file = fs::read_to_string("weather-stations.txt").unwrap();
	let mut weather_stations:Vec<String> = vec![];
	let mut temp_string = String::new();

	for (_,char) in file.chars().enumerate(){
		match char {
			';' => {
				weather_stations.push(temp_string.clone());
				temp_string.clear()
			},
			'\r' | '\n' => {

			},
			_ => {
				temp_string.push(char);
			}
		}
	}

	weather_stations
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    count: u32,
}