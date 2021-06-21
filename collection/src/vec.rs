#[derive(Debug)]
pub enum Complex {
    Int(i32),
    Float(f32),
    String(String),
}
pub fn use_vector() {
    // create vector
    let mut vec1:Vec<i32> = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.push(4);
    println!("vec1:{:?}",vec1);
    // 2.create vector
    let vec2 = vec![1,2,3,4,5];
    println!("vec2:{:?}",vec2);
    

    // 1. read
    
    let first = &vec1[0];
    // vec1.push(5);
    println!("first:{}",first);
    // println!("out of index:{}",&vec1[200]);
    // 2. read
    let first2 = vec1.get(0);
    match first2 {
        Some(val) => println!("first2:{}",val),
        None => println!("none"),
    }
    let out_range = vec2.get(200);
    match out_range {
        Some(val) => println!("outRange:{}",val),
        None => println!("out of range"),
    }

    // foreach vector
    //
    let vec3 = vec![1,2,3,4,5];
    for i in &vec3 {
        println!("{}",i);
    }

    // for i in 0..vec3.len() {
    //     println!("this is {} elements : {}",i,match vec3.get(i) {
    //         Some(val) => val,
    //         None => -1,
    //     });
    // }
    
    // foreach modify
    //
    let mut vec4 = vec3;
    for i in &mut vec4 {
        *i += 100;
    }
    println!("vec4:{:?}",vec4);
    for i in 0..vec4.len() {
        vec4.pop();
    }
    println!("after pop vec4:{:?}",vec4);


    // vector enum 
    let vec5 = vec![
        Complex::String(String::from("111")),
        Complex::Float(520.1314),
        Complex::Int(11),
    ];
    let complex1 = &vec5[0];
    if let Complex::String(val) = complex1 {
        println!("complex1:{}",val);
    }
    
    println!("vec5:{:?}",vec5);

    


}
