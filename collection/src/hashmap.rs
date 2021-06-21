use std::collections::HashMap;
pub fn use_hashmap() {
    // 1. create
    let mut map1 = HashMap::new();
    map1.insert(String::from("Blue"),10);
    map1.insert(String::from("Yellow"),20);
    let string1 = String::from("pink");
    let int1 = 30;
    map1.insert(string1,int1);
    // map1.insert(1,2);
    println!("map1:{:?}",map1);
    // println!("string1:{}",string1);
    println!("int1:{}",int1);
    // 2.create two vector
    let vec1 = vec![1,2,3,4,5];
    let vec2 = vec![
        String::from("111") ,
        String::from("222"),
        String::from("333"),
        String::from("444"),
        String::from("555"),
    ];
    let map2:HashMap<_,_> = vec1.iter().zip(vec2.iter()).collect();
    // let map2 = vec1.iter().zip(vec2.iter()).collect();
    println!("map2:{:?}",map2);

}
