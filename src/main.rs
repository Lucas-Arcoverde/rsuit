// rSutil by Lucas Arcoverde
// rsutil is a collection of many linux/unix commands in a single file written in Rust.
//
//

use std::fs;

fn usage() {
    println!("rSutil - Lucas Arcoverde");
    println!("version 26.11.05\n");

    println!("Usage:");
    println!("\trsu <command> <args>\n");

    println!("Example:");
    println!("\trsu ls /home/user/pics\n\n");

    println!("See all rsutils commands by typing: rsu listcmds");
}

// Clear terminal screen
fn clear() {
    print!("\x1B[2J\x1B[H");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        usage();
        return;
    }

    let program = &args[1];

    if program == "clear" {
        clear();
    }
}
