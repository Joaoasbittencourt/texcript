use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() {

	let start_pattern = "${";
	let end_pattern = "}";

	let args = Cli::from_args();
	let content = std::fs::read_to_string(&args.path).expect("could not read file");

	for line in content.lines() {
		if line.contains(start_pattern) {
			println!("{}", line);
		}
	}
}
