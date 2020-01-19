use clap::{App, Arg, SubCommand};
use std::process::Command;

/* Macro for concise initialization of hashmap */
macro_rules! map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

/* Macro for creating a vector for key bindings */
macro_rules! keys {
    ($( $key: expr => $desc: expr ),*) => {{
         let mut keys: Vec<(&str, &str)> = ::std::vec::Vec::new();
         $( keys.push(($key, $desc)); )*
         keys
    }}
}

pub const KEY_BINDINGS: &'static [(&str, &str)] = &[
	("'?', f1", "help"),
	("right/left, h/l", "switch between blocks"),
	("up/down, k/j", "scroll up/down [selected block]"),
	("pgup/pgdown", "scroll up/down [kernel activities]"),
	("</>", "scroll up/down [module information]"),
	("ctrl-t/b, home/end", "scroll to top/bottom [module list]"),
	("\\, tab, backtab", "show the next kernel information"),
	("/, s, enter", "search a kernel module"),
	("+, i, insert", "load a kernel module"),
	("-, u, backspace", "unload the kernel module"),
	("x, b, delete", "blacklist the kernel module"),
	("y/n", "execute/cancel the command"),
	("c/v", "copy/paste"),
	("r, f5", "refresh"),
	("q, ctrl-c/d, esc", "quit"),
];

/* ASCII format of the project logo */
const ASCII_LOGO: &str = "
 ``    ````````````    ````   ```````````    ```````````
:NNs `hNNNNNNNNNNNNh` sNNNy   yNNNNNNNNNN+   dNNNNNNNNNN:
/MMMydMMyyyyyyydMMMMdhMMMMy   yMMMyyyhMMMo   dMMMyyydMMM/
/MMMMMMM`      oMMMMMMMMMMy   yMMM`  -MMMo   dMMN   /MMM/
/MMMs:::hhhs   oMMM+:::MMMNhhhNMMMdhhdMMMmhhhNMMN   /MMM/
:mmm/   dmmh   +mmm-  `mmmmmmmmmmmmmmmmmmmmmmmmmd   /mmm:
 ```    ```     ```    ``````````````````````````    ```";

/**
 * Parse command line arguments using clap.
 *
 * @return ArgMatches
 */
pub fn parse_args() -> clap::ArgMatches<'static> {
	App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.usage("Press '?' while running the app to see key bindings")
		.before_help(ASCII_LOGO)
		.arg(
			Arg::with_name("color")
				.short("c")
				.long("color")
				.value_name("COLOR")
				.default_value("darkgray")
				.help("Set the main color using hex or color name")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("rate")
				.short("t")
				.long("tickrate")
				.value_name("MS")
				.default_value("250")
				.help("Set the refresh rate of the terminal")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("reverse")
				.short("r")
				.long("reverse")
				.help("Reverse the kernel module list"),
		)
		.subcommand(
			SubCommand::with_name("sort")
				.about("Sort kernel modules")
				.arg(
					Arg::with_name("size")
						.short("s")
						.long("size")
						.help("Sort modules by their sizes"),
				)
				.arg(
					Arg::with_name("name")
						.short("n")
						.long("name")
						.help("Sort modules by their names"),
				),
		)
		.get_matches()
}

/**
 * Execute a operating system command and return its output.
 *
 * @param  cmd
 * @param  cmd_args
 * @return Result
 */
pub fn exec_cmd(cmd: &str, cmd_args: &[&str]) -> Result<String, String> {
	match Command::new(cmd).args(cmd_args).output() {
		Ok(output) => {
			if output.status.success() {
				Ok(String::from_utf8(output.stdout)
					.expect("not UTF-8")
					.trim_end()
					.to_string())
			} else {
				Err(String::from_utf8(output.stderr)
					.expect("not UTF-8")
					.trim_end()
					.to_string())
			}
		}
		Err(e) => Err(e.to_string()),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_parse_args() {
		let matches = parse_args();
		assert_ne!(0, matches.args.len());
		assert_eq!(true, matches.usage.unwrap().lines().count() > 1);
	}
	#[test]
	fn test_exec_cmd() {
		assert_eq!("test", exec_cmd("printf", &["test"]).unwrap());
		assert_eq!(
			"true",
			exec_cmd("sh", &["-c", "test 10 -eq 10 && echo 'true'"]).unwrap()
		);
		assert_eq!(
			"err",
			exec_cmd("cat", &["-x"]).unwrap_or(String::from("err"))
		);
	}
}
