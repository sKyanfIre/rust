pub fn use_lifecycle() {
    // let a;
    // {
    //     let b = String::from("zzz");
    //     a = &b;
    //     println!("a :{}",a);
    // }
    // println!("a :{}",a);
    let people1 = People {
        id:1,
        name:"zzz",
        age:2,
    };
    println!("people1:{:?}",people1);
    let string1 = String::from("123456789");
    let str1 = &string1[..];
    let str2 = &string1[..];

    println!("{}",lifecycle1(str1,str2));


}
pub fn lifecycle1<'a>(param1:&'a str,param2:&'a str)->&'a str {
    // let tmp = String::from("yyyyyyy");
    // let tmp1 = &tmp[..];
    // let tmp = "zzz";
    if param1.len() > param2.len(){
        return param1;
    }
    param2
}
#[derive(Debug)]
pub struct People<'a> {
    pub id:i32,
    pub name:&'a str,
    pub age:i32,
}
