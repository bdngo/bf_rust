use std::{env, process};
use bf_rust::Program;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let machine = Program::new(&argv);
    match machine {
        Ok(mut p) => {
            match p.run() {
                Ok(s) => print!("{}", s),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                },
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        },
    }
}
