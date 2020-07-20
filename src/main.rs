use std::fs::File;

use std::env;


mod helper;
mod disk;


use disk::retrieve_boot;
use disk::boot::Boot;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_loc = args[1].as_str();
    let mut file: File = File::open(file_loc).expect("Expected Path to Disk File");

    let boot_result : Result< Box::<dyn Boot>, String> = retrieve_boot(&mut file);

    match boot_result
    {
        Ok(boot) => boot.print_boot_level(),
        Err(err) => println!("{}", err)
    }
}
