use std::process::exit;
use clap::Parser;
use semver::Version;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
	/// The base and target version seperated by `:`  to compare like `1.1.0:1.2.0`
	#[clap(short, long)]
	cmp: String,

	/// long or short output, defaults to long
	#[clap(short, long)]
	verbose: bool,

	/// if the bool representation should use `true` instead of 1
	#[clap(short, long)]
	text: bool,

	/// if the program should represent a date via the exit code
	#[clap(short, long)]
	exit: bool,
}

fn main() {
	let args = Args::parse();

	let translate_long = |long: String| {
		let mut split =  long.split('.').collect::<Vec<&str>>();
		if split.len() > 3 {
			split.reverse();
			let mut chunks = split.chunks(3).next().unwrap().to_vec();
			chunks.reverse();
			chunks.join(".").to_owned()
		} else {
			long
		}
	};

	let split = &args.cmp.split(':').collect::<Vec<&str>>();
	let base = Version::parse(&translate_long(split[0].to_owned())).unwrap();
	let target = Version::parse(&translate_long(split[1].to_owned())).unwrap();

	match () {
		_ if args.verbose => {
		let age = || return if base > target { "newer" } else { "older" };

		println!("Base version {base} is {} than {target}", age());
		}
		_ if args.text => {
			println!("{}", base < target);
		}
		_ if args.exit => {
			exit(!(base < target) as i32);
		}
		_ => {
			println!("{}", (base < target) as u8);
		}
	}
}