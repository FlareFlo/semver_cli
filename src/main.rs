use clap::Parser;
use semver::Version;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
	/// The base version given
	#[clap(short, long)]
	base: String,

	/// The target version in relation the base
	#[clap(short, long)]
	target: String,

	/// long or short output, defaults to long
	#[clap(short, long)]
	long_output: bool,
}

fn main() {
	let args = Args::parse();

	let base = Version::parse(&args.base).unwrap();
	let target = Version::parse(&args.target).unwrap();

	if args.long_output {
		let age = |cmp| return if cmp { "newer" } else { "older" };

		println!("Base version {base} is {} than {target}", age(base > target));
	} else {
		println!("{}", base < target);
	}
}