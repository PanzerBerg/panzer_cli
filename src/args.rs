use clap::ArgMatches;
pub mod process_args;

pub fn help_all() {
    println!("\n\r      '\
                \r       _\\______
                \r      /        \\========
                \r ____|__________\\_____
                \r/ ___________________ \
                \r\\/ _===============_ \\/
                \r  '-===============-'\n
                \rPanzer CLI help:\n
                \rARGUMENTS:
                \rNOTE: Argument values can also be 'all' that returns a list instead of a single value.
                \r'--p' or '--processes' <VALUE>    'Returns a current running process in the system.'
                \r'--c' or '--components' <VALUE>   'Returns a hardware component information and temperature.'
                \n
                \rNOTE: Arguments can be followed by '-v' that prints the prints a full report of the steps the CLI is taking.\n\n");
}

pub fn help_with_arg() {
    println!("\nERROR:
                \rCan't execute '--help' with multiple arguments.\n
                \rYou can either call '--help' on a single argument, or without any.
                \n
                \r--<ARGUMENT> --help     'Shows the help for that argument'\n
                \r--help                  'Shows a help for all the arguments available'\n")
}

pub fn help_verbose(matches: ArgMatches) {
    println!("Calls a argument module with verbose
                \rArgs: {:?}", matches.args);
}

pub fn help_single_arg(matches: ArgMatches) {
    println!("Help with argument! (Call help inside the arg module)
            \rArgs: {:?}", matches.args);
}

pub fn call_arg_module(matches: ArgMatches) {
    if matches.is_present("processes") {
        process_args::processes_main(matches);
    }
}