
use num_cpus;
use std::env;

fn main() {
    println!("\n***** CPU info & PWD ****\n");
   
   let log_cup_count= num_cpus::get();
    println!("\nNumber of pysical CPUs is: {}\n", log_cup_count); 

    let phy_cpu_count = num_cpus::get_physical();
    println!("\nNumber of logical CPUs is: {}\n", phy_cpu_count);


    let pwd = env::current_dir().unwrap();
    println!("\nCurrent Working directory is: {:?}\n", pwd);
    //println!("Current directory is: {}", pwd.display());
}
