pub fn use_string (){
    let str = String::from("zzz");
    let mut str2 = String::new();
    str2.push_str("zzz");
    str2.push('h');
    let slice1 = "and yyy";
    str2.push_str(slice1);
    println!("str2:{}",str2);
    println!("slice1:{}",slice1);

    let str3 = " become rich";
    let str4 = str + str3;
    println!("str4:{}",str4);

    // println!("str:{}",str);
    println!("str3:{}",str3);
    let str5 = str4 + &str2;
    println!("str5:{}",str5);
    println!("str2:{}",str2);

    let slice2 = "zzz";
    let slice3 = "yyy";
    // let slice4 = &slice2 + &slice3;

    let str6 = format!("{} {} {}",slice1,slice2,slice3);
    println!("str6:{}",str6);
    println!("slice1:{}",slice1);
    println!("slice3:{}",slice3);
    let str7 = String::from("zzz");
    let str8 = String::from("and");
    let slice4 = "yyy";
    let str9 = format!("{} {} {}",str7,str8,slice4);
    println!("str9:{}",str9);
    println!("str7:{}",str7);
    println!("str8:{}",str8);
    println!("slice4:{}",slice4);

    // string length
    //
    let len1 = String::from("zzzz").len();
    println!("len1:{}",len1);
    let len2 = String::from("一一一一").len();
    let string1 = String::from("zzz");
    let string2 = String::from("一一一一");
    println!("len2:{}",len2);
    println!("english slice :{}",&string1[0..1]);
    println!("chinese slice:{}",&string2[0..3]);

    // chars
    println!("english chars ....");
    for i in string1.chars() {
        println!("{}",i);
    }
    println!("chinese chars ....");
    for i in string2.chars() {
        println!("{}",i);
    }

    // bytes
    println!("englisth bytes...");
    for i in string1.bytes() {
        println!("{}",i);
    }
    println!("chinses bytes ....");
    for i in string2.bytes() {
        println!("{}",i);
    }
    
}
