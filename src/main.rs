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
}

fn main() {
	let args = Args::parse();

	let split = &args.cmp.split(':').collect::<Vec<&str>>();
	let base = Version::parse(&split[0]).unwrap();
	let target = Version::parse(&split[1]).unwrap();

	if args.verbose {
		let age = || return if base > target { "newer" } else { "older" };

		println!("Base version {base} is {} than {target}", age());
	} else {
		println!("{}", base < target);
	}
}