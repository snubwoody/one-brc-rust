use std::{
	collections::{btree_map::{OccupiedEntry, VacantEntry}, hash_map::Entry, HashMap}, 
	fs::File, 
	io::{BufRead, BufReader}, time::Instant
};

fn main() {
	let moment = Instant::now();
	let measurements: HashMap<String, Vec<f32>> = generate_measurements();
	println!("elapsed: {:?}",moment.elapsed());
}

fn generate_measurements() -> HashMap<String,Vec<f32>>{
	let file = File::open("measurements.txt").unwrap();
	let buffer = BufReader::new(file);
	let mut measurements:HashMap<String, Vec<f32>> = HashMap::new();

	for (_,line) in buffer.lines().enumerate(){
		let temp_line = line.unwrap();
		let values:Vec<&str> = temp_line.split(';').collect();
		let entry = measurements.entry(String::from(values[0]));
		match entry {
			Entry::Vacant(vacant_entry) => {
				measurements.insert(String::from(values[0]), vec![values[1].parse::<f32>().unwrap()]);
			}
			Entry::Occupied(occupied_entry) => {
				occupied_entry.into_mut().push(values[1].parse::<f32>().unwrap());
			}
		}
	}

	measurements
}
