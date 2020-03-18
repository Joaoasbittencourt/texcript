use structopt::StructOpt;
use std::fs::{File};
use std::io::Write;
use std::collections::HashMap;

static START_PATTERN: &str = "${";
static END_PATTERN: &str = "}";

#[derive(StructOpt)]
struct Cli {
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,

	#[structopt(parse(from_os_str))]
	params_path: std::path::PathBuf,

	#[structopt(parse(from_os_str))]
	out_path: std::path::PathBuf,
}

fn make_token(var_name: &str) -> String {
	return format!("{}{}{}", START_PATTERN, var_name.trim(), END_PATTERN);
}

fn main() {

	let args = Cli::from_args();

	// input file
	let input_content = std::fs::read_to_string(&args.path);
	let input_data = match input_content {
		Ok(content) => { content }
		Err(error) => { panic!("Error: {}", error) }
	};

	// params file
	let params_content = std::fs::read_to_string(&args.params_path);
	let params_data = match params_content {
		Ok(content) => { content }
		Err(error) => { panic!("Error: {}", error) }
	};

	let mut params = HashMap::new();

	for line in params_data.lines() {
		if line.contains("=") {

			let items: Vec<String> = line.split("=").map(|s| s.to_string()).collect();

			let param = items[0].to_string();
			let value = items[1].to_string();

			params.insert(param, value);
		}
	}

	let output_file_name = &args.out_path;
	let mut data = input_data;

	for (param, value) in params {
		let token = make_token(&param);
		data = data.replace(&token, &value);

	}

	let mut file = File::create(output_file_name).expect("Unable to create file");

	file.write_all(data.as_bytes()).expect("Unable to write to file");
}
