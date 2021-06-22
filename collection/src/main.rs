pub mod vec;
pub mod string;
pub mod hashmap;
pub mod generic;
pub mod error;
pub mod mytrait;
pub mod lifecycle;
fn main() {
    // println!("Hello, world!");
    vec::use_vector();
    println!("-------------------------------");
    string::use_string();
    println!("-------------------------------");
    hashmap::use_hashmap();
    println!("-------------------------------");
    error::use_error();
    println!("-------------------------------");
    generic::use_generic();
    println!("-------------------------------");
    mytrait::use_trait();
    println!("---------------------------------");
    lifecycle::use_lifecycle();
}
