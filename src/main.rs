use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() {

	println!();
	let start_pattern = "${";
	let end_pattern = "}";

	let args = Cli::from_args();
	let result = std::fs::read_to_string(&args.path);

	let content = match result {
		Ok(content) => { content }
		Err(error) => {
			panic!("Error: {}", error);
		}
	};

	for line in content.lines() {
		if line.contains(start_pattern) {
			let start_index = line.find(start_pattern);
			let line_index = line.find(end_pattern);

			// get the variables from the variables files
			// get where to replace
			// create abither file
		}
	}
}
