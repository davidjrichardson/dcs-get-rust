extern crate rustc_serialize;
extern crate regex;

use regex::Regex;

use std::str::FromStr;
use std::process;
use std::env;

// Version number specified in the cargo crate
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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

#[derive(Debug, RustcDecodable)]
struct ListArgs {
    flag_type: Option<String>,
}

const USAGE_SEARCH: &'static str = "
dcs-get search: Searches the package list for a package or packages

Usage:
	dcs-get search <query>
";

#[derive(Debug, RustcDecodable)]
struct SearchArgs {
    arg_query: String,
}

const USAGE_INSTALL: &'static str = "
dcs-get install: Installs the package(s) provided onto the current system

Usage:
	dcs-get install [-v] [-d] <package>...

Options:
	-v, --verbose  Verbose output of the installation process
	-d, --dry-run  Outputs the files to be installed without installation
";

#[derive(Debug, RustcDecodable)]
struct InstallArgs {
    flag_v: bool,
    flag_d: bool,
    arg_package: Vec<String>,
}

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

#[derive(Debug, RustcDecodable)]
struct RemoveArgs {
    flag_v: bool,
    flag_d: bool,
    flag_p: bool,
    flag_f: bool,
    arg_package: Vec<String>,
}

const USAGE_GENSYMLINKS: &'static str = "
dcs-get gensymlinks: Generates the symlinks for any new packages, linking them into dcs-get

Usage:
	dcs-get gensymlinks <package>
";

#[derive(Debug, RustcDecodable)]
struct GensymlinksArgs {
    arg_package: String,
}

const USAGE_UPLOAD: &'static str = "
dcs-get upload: Uploads the given package for approval into the dcs-get repository

Usage:
	dcs-get upload [--metadata <path>] <package>

Options:
	--metadata <path>  Path to the package metadata file
";

#[derive(Debug, RustcDecodable)]
enum Command {
    Help,
    List,
    Search,
    Install,
    Remove,
    Gensymlinks,
    Upload,
}

// Coerce String into Command enum
impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Command, ()> {
        match s {
            "help" => Ok(Command::Help),
            "list" => Ok(Command::List),
            "search" => Ok(Command::Search),
            "install" => Ok(Command::Install),
            "remove" => Ok(Command::Remove),
            "gensymlinks" => Ok(Command::Gensymlinks),
            "upload" => Ok(Command::Upload),
            _ => Err(()),
        }
    }
}

fn print_help_with_msg(msg: String, help: String, exit_code: i32) {
    println!("{}", msg);
    println!("{}", help);
    process::exit(exit_code);
}

fn print_help(help: String, exit_code: i32) {
    println!("{}", help);
    process::exit(exit_code);
}

fn help_command(args: &Vec<String>) {
    match args.len() {
        0 => {
            print_help(String::from(USAGE_MAIN), 0);
        }
        1 => {
            match args[0].parse::<Command>() {
                Ok(arg) => {
                    let usage = match arg {
                        Command::Help => USAGE_HELP,
                        Command::List => USAGE_LIST,
                        Command::Search => USAGE_SEARCH,
                        Command::Install => USAGE_INSTALL,
                        Command::Remove => USAGE_REMOVE,
                        Command::Gensymlinks => USAGE_GENSYMLINKS,
                        Command::Upload => USAGE_UPLOAD,
                    };

                    print_help(String::from(usage), 0);
                }
                Err(_) => {
                    print_help_with_msg(format!("{:?} isn't a valid command", args[0]),
                                        String::from(USAGE_MAIN),
                                        1);
                }
            }
        }
        _ => {
            print_help_with_msg(String::from("Help only works with 0 or 1 commands"),
                                String::from(USAGE_MAIN),
                                1);
        }
    }
}

fn list_command(args: ListArgs) {
    println!("{:?}", args);

    process::exit(0);
    // TODO
}

fn search_command(args: SearchArgs) {
    println!("{:?}", args);

    process::exit(0);
    // TODO
}

fn install_command(args: InstallArgs) {
    println!("{:?}", args);

    process::exit(0);
    // TODO
}

fn remove_command(args: RemoveArgs) {
    println!("{:?}", args);

    process::exit(0);
    // TODO
}

fn gensymlinks_command(args: GensymlinksArgs) {
    println!("{:?}", args);

    process::exit(0);
    // TODO
}

fn parse_args(mut args: Vec<String>) {
    let command = args.remove(0);

    match command.parse::<Command>() {
        Ok(cmd) => {
            match cmd {
                Command::Help => {
                    help_command(&args);
                }
                Command::List => {
                    // Search for the '--type' flag
                    let type_index: i32 = args.iter()
                                              .position(|p| p == "--type")
                                              .map(|x| x as i32)
                                              .unwrap_or(-1);

                    let mut command_args = ListArgs { flag_type: None };

                    // If it finds the flag, parse it or throw an error if not present
                    if type_index > -1 {
                        let type_str = args.iter()
                                           .nth(type_index as usize + 1);
                        match type_str {
                            Some(t) => command_args.flag_type = Some(t.clone()),
                            None => {
                                print_help_with_msg(String::from("Please specify a type after \
                                                                  the --type flag"),
                                                    String::from(USAGE_LIST),
                                                    1);
                            }
                        }
                    }

                    // Execute the command
                    list_command(command_args);
                }
                Command::Search => {
                    // Use the first argument for the query
                    match args.get(0) {
                        Some(q) => {
                            let command_args = SearchArgs { arg_query: q.clone() };

                            search_command(command_args);
                        }
                        None => {
                            print_help_with_msg(format!("Please provide a search query"),
                                                String::from(USAGE_SEARCH),
                                                1)
                        }
                    }
                }
                Command::Install => {
                    let verbose_index = args.iter()
                                            .position(|p| p == "-v" || p == "--verbose");
                    let dry_run_index = args.iter()
                                            .position(|p| p == "-d" || p == "--dry-run");

                    // Removes all --verbose/--dry-run/etc. flags
                    let mut packages = args.to_vec();
                    packages.retain(|p| p != "-v" && p != "--verbose");
                    packages.retain(|p| p != "-d" && p != "--dry-run");
                    // Necessary copy?
                    let residual = packages.to_vec();

                    // Check if there are any invalid options left i.e.: --v, -f
                    let opt_regex = Regex::new(r"^-+\w+").unwrap();
                    let invalid_args: Vec<_> = residual.iter()
                                                       .filter(|p| opt_regex.is_match(p))
                                                       .collect();
                    // Exit if any invalid args are found
                    if !invalid_args.is_empty() {
                        print_help_with_msg(format!("Found invalid aruments: {:?}", invalid_args),
                                            String::from(USAGE_INSTALL),
                                            1);
                    }

                    let command_args = InstallArgs {
                        flag_d: dry_run_index.is_some(),
                        flag_v: verbose_index.is_some(),
                        arg_package: packages,
                    };

                    install_command(command_args);
                }
                Command::Remove => {
                    let verbose_index = args.iter()
                                            .position(|p| p == "-v" || p == "--verbose");
                    let dry_run_index = args.iter()
                                            .position(|p| p == "-d" || p == "--dry-run");
                    let force_index = args.iter()
                                          .position(|p| p == "-f" || p == "--force");
                    let prune_index = args.iter()
                                          .position(|p| p == "-p" || p == "--prune-deps");

                    // Removes all --verbose/--dry-run/etc. flags
                    let mut packages = args.to_vec();
                    packages.retain(|p| p != "-v" && p != "--verbose");
                    packages.retain(|p| p != "-d" && p != "--dry-run");
                    packages.retain(|p| p != "-f" && p != "--force");
                    packages.retain(|p| p != "-p" && p != "--prune-deps");
                    // Necessary copy?
                    let residual = packages.to_vec();

                    // Check if there are any invalid options left i.e.: --v, -f
                    let opt_regex = Regex::new(r"^-+\w+").unwrap();
                    let invalid_args: Vec<_> = residual.iter()
                                                       .filter(|p| opt_regex.is_match(p))
                                                       .collect();
                    // Exit if any invalid args are found
                    if !invalid_args.is_empty() {
                        print_help_with_msg(format!("Found invalid aruments: {:?}", invalid_args),
                                            String::from(USAGE_REMOVE),
                                            1);
                    }

                    let command_args = RemoveArgs {
                        flag_d: dry_run_index.is_some(),
                        flag_v: verbose_index.is_some(),
                        flag_f: force_index.is_some(),
                        flag_p: prune_index.is_some(),
                        arg_package: packages,
                    };

                    remove_command(command_args);
                }
                Command::Gensymlinks => {
                    // Use the first argument for the query
                    match args.get(0) {
                        Some(q) => {
                            let command_args = GensymlinksArgs { arg_package: q.clone() };

                            gensymlinks_command(command_args);
                        }
                        None => {
                            print_help_with_msg(format!("Please provide a package to generate \
                                                         symlinks for"),
                                                String::from(USAGE_GENSYMLINKS),
                                                1)
                        }
                    }
                }
                _ => println!("{:?}", cmd),
            }
        }
        Err(_) => {
            // Deal with the version case
            if command == "-v" || command == "--version" {
                println!("dcs-get version {}", VERSION);
                process::exit(0);
            } else {
                print_help_with_msg(format!("{:?} is not a valid command", command),
                                    String::from(USAGE_MAIN),
                                    1);
            }
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    match args.len() {
        0 | 1 => println!("{}", USAGE_MAIN),
        _ => {
            // Remove the dcs-get command
            args.remove(0);
            parse_args(args);
        }
    }
}
