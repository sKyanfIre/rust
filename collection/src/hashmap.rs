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
    // hashmap get
    let string2 = String::from("pink");
    let value1 = map1.get(&string2);
    println!("value1:{:?}",value1);
    
    // hashmap foreach
    // for i in &map1 {
    //     println!("{}：{}",i.0,i.1);
    // }
    for (key,val) in &map1 {
        println!("{},{}",key,val);
    }
    // hashmap update

    let mut map3 = HashMap::new();
    map3.insert(1,String::from("111"));
    map3.insert(1,String::from("222"));
    println!("map3:{:?}",map3);
    // entry
    map3.entry(1).or_insert(String::from("333"));
    map3.entry(2).or_insert(String::from( "222222" ));
    println!("map3:{:?}",map3);
    
    let text = "hello world wonderful";
    let mut map4 = HashMap::new();
    for i in text.split_whitespace() {
        let count = map4.entry(i).or_insert(0);
        *count += 1;
    }
    let mut map5 = HashMap::new();
    println!("map4:{:?}",map4);
    for i in text.chars(){
        let count = map5.entry(i).or_insert(0);
        *count += 1;
    }
    println!("map5:{:?}",map5);
    println!("h:count:{:?}",map5.get(&'h'));

    

}
