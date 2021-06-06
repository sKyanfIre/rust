fn main() {
// tuple
// let tuple2:(i32,i32,&str,char) = (10,520,"zzz",'a'); 
let str = String::from("zzz");
// let tuple2:(i32,i32,&str,char) = (10,520,"zzz",'a'); 
// let tuple2:(i32,i32,String,char) = (10,520,str,'a'); 
let tuple2:(i32,i32,char) = (10,520,'a'); 
println!("tuple2:{},{},{}",tuple2.0,tuple2.1,tuple2.2);
let tuple3:(i32,i32,String,char) = (10,520,str,'a'); 
println!("tuple3:{},{},{},{}",tuple3.0,tuple3.1,tuple3.2,tuple3.3);
let tuple4:(i32,i32,&str,char) = (10,520,"zzz",'a'); 
println!("tuple4:{},{},{},{}",tuple4.0,tuple4.1,tuple4.2,tuple4.3);
let (x,y,z,a) = tuple4;
println!("tuple5:{},{},{},{}",x,y,z,a);

 // array
 // 1.
 let array1 = [1,2,3,4];
 println!("array1:{:?}",array1);
// 2.
let array2:[i32;4] = [1,2,3,4];

println!("array2:{:?}",array2);

//3.
let array3 = [3;4];
println!("array3:{:?}",array3);
//4.
// for arr in array3.iter

let param1 = 4;
let param2 = 5;
println!("{} + {} = {}",param1,param2,sum(param1,param2));

let string = String::from("i miss you pig!");
let index = 6;
// println!("{} substring is {},index = {}",string,sub_string(string,index),index);

// if statement
let random = 80;
    if random < 60{
        println!("random :{}",'D');
    }else if random < 70  {
        println!("random:{}",'C');
    }else if random < 90 {
        println!("random:{}",'B');
    }else {
        println!("random:{}",'A');
    }
// loop
//
let mut loop_inx = 0;
loop {
    loop_inx += 1;
    println!("loop_idx:{}",loop_inx);
}

}

fn sum(param1:i32,param2:i32)->i32{
    param1 + param2
}
// fn sub_string(s:String,index:i32)->String {
//    &s[..index] 
// }
