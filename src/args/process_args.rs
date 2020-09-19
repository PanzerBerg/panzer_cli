use clap::ArgMatches;
use sysinfo::{SystemExt, ProcessExt};

pub fn processes_main(matches: ArgMatches) {
    let mut sys = sysinfo::System::new_all();
    let processes_arg = matches.value_of("processes");
    
    sys.refresh_all();
    //println!("{:?}", processes_arg);

    if processes_arg == Some("all") {
        all_process(sys);
    } else {
        single_process(sys, processes_arg.unwrap());
    }
}

fn all_process(sys: sysinfo::System) {
    let processes = sys.get_processes();
    println!("\nTotal number processes running: {}\n\n", processes.len());

    for (pid, proc_) in processes {
        println!("#{}:    '{}' => status: {:?}", pid, proc_.name(), proc_.status());
    }
}

fn single_process(sys: sysinfo::System, arg: &str) {
    let processes = sys.get_process_by_name(arg);

    if processes.len() == 0 {
        println!("\nNo process named {} actively running.\n", arg);
    } else {
        println!("\nTotal number of processes named {} found: {}\n\n", arg, processes.len());

        for process in processes {
            println!("#{}:      '{}'    =>  Status: {}\n\n
                        \rProcess path:             {}
                        \rProcess CPU usage:        {}
                        \rProcess memory usage:     {}
                        \rVirtual memory usage:     {}
                        \rStart time:               {}
                        \rWritten in disk:          {}/{}
                        \rRead in disk:             {}/{}\n", 
                        process.pid(), process.name(), process.status(), process.exe().display().to_string(), 
                        process.cpu_usage(), process.memory(), process.virtual_memory(), process.start_time(), 
                        process.disk_usage().written_bytes, process.disk_usage().total_written_bytes,
                        process.disk_usage().read_bytes, process.disk_usage().total_read_bytes);
        };
    }

}