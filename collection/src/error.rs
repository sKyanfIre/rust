use std::io;
use std::fs::File;
use std::io::Read;
use std::fs;
pub fn use_error(){
    // panic!("game over!");
    let vec1 = vec![1,2,3];
    // vec1[100];
    vec1[0];
    println!("vec1:{:?}",vec1);
    let vec2 = vec![String::from("1"),String::from("2")];
    &vec2[0];
    println!("vec2:{:?}",vec2);

    // Result 
    let result = File::open("/home/zzz/git/rust/collection/Cargo.toml");
    let result = match result {
        Ok(f) => f,
        Err(f) => panic!("{}",f),
    };
    // handle different error
    
    let result = File::open("/home/zzz/git/rust/collection/tmp.txt");
    let result = match result {
        Ok(f) => f,
        Err(f) => match f.kind(){
            std::io::ErrorKind::NotFound => match File::create("/home/zzz/git/rust/collection/tmp.txt") {
                Ok(f) => f,
                Err(f) => panic!("{}",f),
            },
            other => panic!("{:?}",other),
        }
    };
   // shortcut 
   // unwrap expect
   
    // let f = File::open("tmp.txt").unwrap();
    // let f = File::open("tmp.txt").expect("failed open tmp.txt");
    propagating();
    shortcut_propagating();
    shortcut_propagating2();
    // match shortcut_propagating3() {
    //     Ok(s) => println!("result :{}",s),
    //     Err(e) => panic!("{}",e),
    // };
    
}

// propagating error
pub fn propagating()->Result<String,io::Error>{
    let f = File::open("tmp.txt");
    let mut f = match f {
        Ok(f) => f,
        Err(e) => return Err(e),
    }; 
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// shortcut propagating error 
pub fn shortcut_propagating() ->Result<String,io::Error>{
    let mut f = File::open("tmp.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
pub fn shortcut_propagating2() ->Result<String,io::Error>{
    let mut s = String::new();
    File::open("tmp.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
pub fn shortcut_propagating3() ->Result<String,io::Error> {
    fs::read_to_string("tmp.txt")
}
