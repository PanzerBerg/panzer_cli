use clap::{Arg, App};
mod args;

fn main() {
    let matches = App::new("Panzer CLI")
                        .version("0.1.0")
                        .author("Matheus Santos")
                        .about("A simples CLI that informs you about your system")
                        .arg(Arg::with_name("processes")
                            .short("p")
                            .long("processes")
                            .value_name("PROCESS")
                            .required(false)
                            .empty_values(true))
                        .arg(Arg::with_name("components")
                            .short("c")
                            .long("components")
                            .value_name("COMPONENT")
                            .required(false)
                            .empty_values(true))
                        .arg(Arg::with_name("help")
                            .short("h")
                            .long("help")
                            .takes_value(false))
                        .arg(Arg::with_name("v")
                            .short("v")
                            .takes_value(false))
                        .get_matches();

    if matches.args.len() >= 3 && matches.is_present("help") && !matches.is_present("v") {
        args::help_with_arg();
    } else if matches.args.len() == 2 && matches.is_present("help") {
        args::help_single_arg(matches);
    } else if matches.args.len() == 1 && matches.is_present("help") {
        args::help_all();
    } else if matches.args.len() == 2 && matches.is_present("v") {
        args::help_verbose(matches);
    } else if matches.args.len() == 1 && !matches.is_present("help") {
        args::call_arg_module(matches);
    }
    
}
