use std::io;

fn main() {
	let mut floors = String::new(); //number of flors in building
	let mut your_floor = String::new(); //your flor u are on

		//getting info form user
	
	println!("Print number of flours in building:");

	io::stdin().read_line(&mut floors)
		.ok()
		.expect("Failed to read line");

	println!("Print the flor your on:");

	io::stdin().read_line(&mut your_floor)
		.ok()
		.expect("Failed to read line");

	let floors: i32 = match floors.trim().parse() {
		Ok(num) => num,
		Err(_) => panic!("NaN"),
		};
	
	let your_floor: i32 = match your_floor.trim().parse() {
		Ok(num) => num,
		Err(_) => panic!("NaN"),
		};

		//calculating part "The Algorythm!"

	match your_floor {
		1 => {
			let perc = 100.0 * (floors - 1) as f32 / floors as f32;
			//output results	
			println!("Your chance is: {}%", &perc);
		},

		_ => {
			let perc = 100.0 / floors as f32 / 2.0;
			//output results	
			println!("Your chance is: {}%", &perc);
		},
	}


}
