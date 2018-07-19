extern crate clap;
extern crate clap_validators;

use clap::{Arg, App};

/// Sample to check for positive numbers
/// 
/// Run with:
/// `cargo run --example example1 -- -n "-12"`
/// or
/// `cargo run --example example1 -- -n "12"`
fn main() {
	let _matches = App::new("My Super Program")
		.arg(Arg::with_name("number")
		.short("n")
		.value_name("NUMBER")
		.help("Some number ...")
		.takes_value(true)
		.validator(clap_validators::num::is_positive))
		.get_matches();

	if let Some(number) = _matches.value_of("number") {
		println!("{} is an awesome value", number);
	}
}