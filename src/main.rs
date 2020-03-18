use structopt::StructOpt;
use std::fs::{File};
use std::io::Write;

static START_PATTERN: &str = "${";
static END_PATTERN: &str = "}";

#[derive(StructOpt)]
struct Cli {
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn make_token(var_name: &str) -> String {
	return format!("{}{}{}", START_PATTERN, var_name, END_PATTERN);
}

fn main() {

	let args = Cli::from_args();
	let result = std::fs::read_to_string(&args.path);

	let content = match result {
		Ok(content) => { content }
		Err(error) => { panic!("Error: {}", error) }
	};

	let value = "João Bittencourt";
	let token = make_token("NAME");
	let generated_path = "text.md";

	let mut file = File::create(generated_path).expect("Unable to create file");
	let data = content.replace(&token, &value);
	file.write_all(data.as_bytes()).expect("Unable to write to file");
}
