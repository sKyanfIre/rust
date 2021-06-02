use structopt::StructOpt;

#[derive(StructOpt)]
struct cli{
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf, 
}
fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std:env::args().nth(2).expect("no path given");
    println!("Hello, world!");
}
