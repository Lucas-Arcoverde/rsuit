// rSutil by Lucas Arcoverde
// rsutil is a collection of many linux/unix commands in a file written in Rust.
//
//

fn usage() {
    println!("rSutil - Lucas Arcoverde");
    println!("version 26.11.05\n");

    println!("Usage:");
    println!("\trsu <command> <flags> <args>\n");

    println!("Example:");
    println!("\trsu ls -la /home/user/pics\n\n");

    println!("See all rsutils commands by typing: rsu listcmds");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        usage();
        return;
    }
}
