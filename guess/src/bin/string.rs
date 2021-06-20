const yezhu:i32 = 11;
const money:f64 = 999_999_999_999_000.29;
fn main() {
    // tuple
    // let tuple2:(i32,i32,&str,char) = (10,520,"zzz",'a'); 
    let str = String::from("zzz");
    // let tuple2:(i32,i32,&str,char) = (10,520,"zzz",'a'); 
    // let tuple2:(i32,i32,string,char) = (10,520,str,'a'); 
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
        println!("random :{}",'d');
    }else if random < 70  {
        println!("random:{}",'c');
    }else if random < 90 {
        println!("random:{}",'b');
    }else {
        println!("random:{}",'a');
    }
    // loop
    //
    let mut loop_inx = 0;
    // loop {
    //     loop_inx += 1;
    //     println!("loop_idx:{}",loop_inx);
    // }
    // shadow
    let tmp = "zzz";
    let tmp = "yyy";
    // let tmp = string::from( "zzz");
    // tmp = string::from(  "yyy" ); 
    // let i = 20;
    // i = 30;
    println!("tmp={}",tmp);
    // println!("i={}",i);
    // mut
    let mut tmp = "zzz";
    tmp = "yyy";
    // tmp = tmp.len();
    let tmp2 = tmp;
    let mut tmp3 = tmp;
    println!("tmp={}",tmp);
    println!("tmp2={}",tmp2);
    println!("tmp3={}",tmp3);
    // const
    println!("const:{}",yezhu);
    println!("const:money:{}",money);
    // bool 
    let flag = true;
    // while flag {
    //     println!("success");
    // }
    // string 
    let str = String::from("zzz");
    println!("str:{}",str);
    let mut i:i8 = 0b0111_1111;
    println!("i:binary:{}",i);
    println!("i:hex:{}",i);
    let mut f:f32 = 1352.124567890;
    println!("f:float:{}",f);
    f= 134.42412412;
    println!("f:float:{}",f);
    // tuple
    let tup2 = (20,20.0,b'a',"zzz");
    let (a,b,c,d) = tup2;
    println!("a:{},b:{},c:{},d:{}",a,b,c,d);
    println!("two: a:{},b:{},c:{},d:{}",tup2.0,tup2.1,tup2.2,tup2.3);
    //bool
    let flag:bool = true;
    println!("flag:{}",flag);
    // array
    let mut arr1 = [1,2,40,4,5];
    println!("arr1:{:#?}",arr1);
    println!("arr1:{:?}",arr1);
    let arr2:[i32;4] = [1,2,3,4];
    println!("arr2:{:?}",arr2);
    let arr3 = [1;200];
    println!("arr3:{:?}",arr3);
    println!("arr1:0:{}",arr1[0]);
    arr1[0] = 2_000_999_143;
    println!("arr1:0:{}",arr1[0]);
    let arr4 = ['a','b','c','d'];
    println!("arr4:{:?}",arr4);
    let idx = 6;
    // println!("arr4:{:?}",arr4[idx]);
    handle(idx);
    println!("idx:{}",idx);
    let flag = false;
    // control statement
    if flag {
        println!("flag:{}",flag);
    }

    let mut flag = flag;
    flag = true;
    let mut idx = 0;
    while flag {
        idx += 1;
        if idx == 999_999 {
            println!("while skip!");
            continue;
        }
        // println!("while:idx:{}",idx);
        if idx == 1_000_000 {
            println!("while end!");
            break;
        }
        let level = if idx == 100_000 {
            'a'
        }else if idx == 500_000{
            'b'
        }else if idx == 800_000 {
            'c'
        }else {
            ' '
        };
        if level != ' ' {
            println!("level:{}",level);
        }
    }
    let mut idx = 1;
    loop {
        println!("loop:idx:{}",idx);  
        idx += 1;
        if idx == 1_0000 {
            break;
        }
    }
    // for 
    let arr5 = [1,2,3,4,5];
    for i in &arr5 {
        println!("for:i:{}",i);
    }
    for i in arr5.iter() {
        println!("for2:i:{}",i);
    }
    for i in 1..10 {
        println!("for3:i:{}",i);
    }
    for i in ( 1..10 ).rev() {
        println!("for4:i:{}",i);
    }
    let s = "abcdef";
    for i in s.chars() {
        println!("for5:i:{}",i);
    }
    // ownership
    let s = String::from("zzz");
    println!("main:s:{}",s);
    let s = handle2(s);
    println!("main:s:{}",s);
    handle3(&s);
    println!("main:s:{}",s);
    let mut s2 = String::from("zzz and ");
    // let s2 = String::from("zzz and ");
    // handle4(&mut s2);
    println!("main:s:{}",s2);
    // let s3 = &s2;
    // let s4 = &s2;
    let s5 = &mut s2;
    // let s6 = &mut s2;
    s5.push_str("111");

    println!("main:s5:{}",s5);
    println!("main:s2:{}",s2);
    // println!("main:s3:{}",s3);
    let mut n1 = String::from("yyy");
    // let n2 = &n1;
    let n2 = &mut n1;
    n2.push_str("and zzz");
    // println!("n1:{}",n1);
    println!("n2:{}",n2);
    handle5(n2);
    let mut arr1 = [1,2,3,4,5];
    
    arr1[0] = 2;
    handle6(arr1);
    println!("arr1:{:?}",arr1);
    
    let arr2 = [1,2,3,4,5];
    let arr3 = arr2;
    println!("arr2:{:?}",arr2);
    println!("arr3:{:?}",arr3);

    let tup1 = (1,2.9,'a',"zzz",String::from("yyy"));
    handle7(&tup1);
    println!("tup1:{:?}",tup1);
    let arr4 = [String::from("1"),String::from("2")];
    handle8(arr4);
    // println!("arr4:{:?}",arr4);
    let tup2 = (1,2,3,4);
    handle9(tup2);
    println!("tup2:{:?}",tup2);
    // slice

    let s = String::from("1234567890");
    let s2 = &s[2..];
    slice1(s2);
    println!("s:{}",s);
    println!("s2:{}",s2);
    slice2(s2);
    println!("s2:{}",s2);
}
fn slice2(s:&str) {
    // s.sub_string("5");
    println!("slice2:s:{}",s);
}
fn slice1(s:&str) {
    // s = "1234";
    println!("slice1:{}",s);
}
fn handle9(tup:(i8,i8,i8,i8)) {
    println!("handle9:tuple:{:?}",tup);
}
fn handle8(arr:[String;2]) {
    println!("handle8:{:?}",arr);
}
fn handle7(tup:&(i32,f32,char,&str,String)) {
    println!("handle7:tup:{:?}",tup);
}
fn handle6(arr:[i32;5]) {
   println!("handle6:{:#?}",arr); 
}
fn handle2(str1:String)->String{
    // str1 =String::from(  "yyy" );
    println!("handle:str1:{}",str1);
    str1
}
// reference
fn handle3(str1:&String) {
    // str1.push_str("zzz");
    println!("handle3:st1:{}",str1);

}
// mutable reference
fn handle4(str1:&mut String) {
    str1.push_str("111");
    println!("handle4:str1:{}",str1);
    let str2 = str1;
}
fn handle5(param1:&String)->&String{
    let s = String::from("tmp str");
    param1
}
fn handle(param1:i32){
    println!("handle:{}",param1);
}

fn sum(param1:i32,param2:i32)->i32{
    param1 + param2
}
// fn sub_string(s:string,index:i32)->string {
//    &s[..index] 
// }
