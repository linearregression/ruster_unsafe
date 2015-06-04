// build.rs
//
// Execute Erlang script to generate API lists and extract config.
//

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
	// FIXME: take env escript if present
	let escript = "escript";

	// emit generated files to OUT_DIR
	let out_dir = env::var("OUT_DIR").unwrap();
	let dst = Path::new(&out_dir);
	println!("OUT_DIR {0}, dst {1}", out_dir, dst);
	Command::new(escript).arg("gen_api.erl").arg(dst).status().unwrap();
	Command::new("ls").arg("-l").arg("*.snippet").status().unwrap();
	Command::new("mv").arg("-f").arg("nif_versions.snippet").arg("src").status().unwrap();
	Command::new("mv").arg("-f").arg("nif_api.snippet").arg("src").status().unwrap();
}
