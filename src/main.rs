extern crate docopt;
extern crate rustc_serialize;

use std::str::FromStr;

use docopt::Docopt;

const USAGE_MAIN: &'static str = "
dcs-get: The University of Warwick Computing Society's package manager.

Usage:
	dcs-get <command> [<args>...]
	dcs-get help [<command>]
	dcs-get (-v | --version)

Commands:
	help		Provides the help prompt (this screen) or the help prompt of the specified package
	list		Lists all packages installed on the current system
	search		Searches the package list for a package or packages
	install 	Installs the package(s) provided onto the current system
	remove		Removes the package(s) provided from the current system
	gensymlinks	Generates the symlinks for any new packages, linking them into dcs-get
	upload		Uploads the given package for approval into the dcs-get repository
";

const USAGE_HELP: &'static str = "
dcs-get help: Show help for any dcs-get command

Usage:
	dcs-get help [<command>]
";

const USAGE_LIST: &'static str = "
dcs-get list: Lists all packages installed on the current system

Usage:
	dcs-get list [--type <type>]

Options:
	--type <type>  The type of package to list
";

const USAGE_SEARCH: &'static str = "
dcs-get search: Searches the package list for a package or packages

Usage:
	dcs-get search <query>
";

const USAGE_INSTALL: &'static str = "
dcs-get install: Installs the package(s) provided onto the current system

Usage:
	dcs-get install [-v] [-d] <package>...

Options:
	-v, --verbose  Verbose output of the installation process
	-d, --dry-run  Outputs the files to be installed without installation
";

const USAGE_REMOVE: &'static str = "
dcs-get remove: Removes the package(s) provided from the current system

Usage:
	dcs-get remove [options] <package>...

Options:
	-v, --verbose  Verbose output of the package removal provess
	-d, --dry-run  Outputs files to be removed without removing them
	-f, --force  Force the removal of the listed package(s), ignoring any errors
	-p, --prune-deps  Prune/remove dependencies of the package(s) to be removed
";

const USAGE_GENSYMLINKS: &'static str = "
dcs-get gensymlinks: Generates the symlinks for any new packages, linking them into dcs-get

Usage:
	dcs-get gensymlinks <package>
";

const USAGE_UPLOAD: &'static str = "
dcs-get upload: Uploads the given package for approval into the dcs-get repository

Usage:
	dcs-get upload [--metadata <path>] <package>

Options:
	--metadata <path>  Path to the package metadata file
";

#[derive(Debug, RustcDecodable)]
struct Args {
	arg_args: Vec<String>,
	arg_command: Option<Command>,
	flag_v:	bool,
	flag_version: bool,
}

#[derive(Debug, RustcDecodable)]
struct HelpArgs {
	arg_command: Option<Command>,
}

#[derive(Debug, RustcDecodable)]
struct ListArgs {
	arg_type: Option<String>,
}

#[derive(Debug, RustcDecodable)]
struct SearchArgs {
	arg_query: String,
}

#[derive(Debug, RustcDecodable)]
struct InstallArgs {
	flag_v: bool,
	flag_d: bool,
	arg_package: Vec<String>,
}

#[derive(Debug, RustcDecodable)]
struct RemoveArgs {
	flag_verbose: bool,
	flag_force: bool,
	flag_prune_deps: bool,
	flag_dry_run: bool,
	arg_package: Vec<String>,
}

#[derive(Debug, RustcDecodable)]
struct GensymlinksArgs {
	arg_package: String,
}

#[derive(Debug, RustcDecodable)]
struct UploadArgs {
    flag_metadata: Option<String>,
    arg_package: String,
}

#[derive(Debug, RustcDecodable)]
enum Command {
	Help, List, Search, Install, Remove, Gensymlinks, Upload
}

// Coerce String into Command enum
impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Command, ()> {
        match s {
        	"help"			=> Ok(Command::Help),
        	"list"			=> Ok(Command::List),
        	"search"		=> Ok(Command::Search),
        	"install"		=> Ok(Command::Install),
        	"remove"		=> Ok(Command::Remove),
        	"gensymlinks"	=> Ok(Command::Gensymlinks),
        	"upload"		=> Ok(Command::Upload),
        	_				=> Err(())
        }
    }
}

fn print_fn_help(args: &Vec<String>) {
	match args.len() {
		0	=> println!("{}", USAGE_MAIN),
		1	=> {
			match args[0].parse::<Command>() {
				Ok(arg)	=> {
					let usage = match arg {
						Command::Help 			=> USAGE_HELP,
						Command::List 			=> USAGE_LIST,
						Command::Search 		=> USAGE_SEARCH,
						Command::Install 		=> USAGE_INSTALL,
						Command::Remove 		=> USAGE_REMOVE,
						Command::Gensymlinks 	=> USAGE_GENSYMLINKS,
						Command::Upload 		=> USAGE_UPLOAD,
					};

					println!("{}", usage);
				},
				Err(_)	=> println!("Argument provided isn't a valid command")
			}
		},
		_	=> panic!("Help only works with 0-1 commands")
	}
}

fn main() {
	let args: Args = Docopt::new(USAGE_MAIN)
                            .and_then(|d| d.options_first(true).decode())
                            .unwrap_or_else(|e| e.exit());
    /*if args.flag_v || args.flag_version {
    	println!("dcs-get version 0.1.0");
    } else {
    	let cmd = match args.arg_command {
    		Some(command)	=> command,
    		None			=> panic!("This shouldn't be reachable :( please let the developers know on github how you broke dcs-get")
    	};

    	match cmd {
    		Command::Help		=> {
    			print_fn_help(&args.arg_args)
    		},
    		Command::List 		=> {
    			let list_args: ListArgs = Docopt::new(USAGE_LIST)
    											 .and_then(|d| d.options_first(true).decode())
    											 .unwrap_or_else(|e| e.exit());
    			println!("{:?}", list_args);
    		},
    		Command::Install 	=> {
				let install_args: InstallArgs = Docopt::new(USAGE_INSTALL)
    												   .and_then(|d| d.options_first(true).decode())
    												   .unwrap_or_else(|e| e.exit());
				println!("{:?}", install_args);
    		},
    		_	=> println!("{:?}", args.arg_args)
    	}
    }*/
}
