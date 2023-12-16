use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("[Error] No input file provided !");
        println!("[Info] Usage : ./yboy <rom.gb>");
        std::process::exit(1);
    }
}
