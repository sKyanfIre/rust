fn main(){
    println!("hello world");
    // tuple
    // let mut str = String::new;
    // &str = "hhhh";
    let tuple4:(i32,f64,char,String) = (741,5.20,'⑤',"hhh".to_string());
    println!("tuple4:{},{},{},{}",tuple4.0,tuple4.1,tuple4.2,tuple4.3);

    let (a,b,c,d) = tuple4;

    println!("second method: tuple4:{},{},{},{}",a,b,c,d);



    // array
    // 1.
    // let array1 = [1,2,4,5];
    // // 2.
    // let array2:[i32;4] = [1,2,4,5];
    // // 3.
    // let array3 = [i32;4];

    // let element = array1[10]; 
}
