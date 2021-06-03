use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli{
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf, 
}
fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std:env::args().nth(2).expect("no path given");
    let args = Cli::from_args();
    // let content = std::fs::read_to_string(&args.path)
    //     .expect("could not read file");
    // println!("pattern:{}",args.pattern);
    // // println!("path:{}",args.path);
    // for line in content.lines() {
    //     if line.contains(&args.pattern){

    //         println!("{}",line);
    //     }
    // }
    let content2 = std::fs::read_to_string(&args.path);
    // match content2 {
    //     Ok(cont) => {println!("File content:{};",cont);}
    //     Err(error) => {println!("Oh noes:{}",error);}
    // }
    let content = match content2 {

        Ok(cont) => {cont}
        Err(error) => {panic!("Can't deal with {},just exit here",error);}
    };
    println!("file content:{}",content);
}
