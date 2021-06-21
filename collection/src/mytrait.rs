use std::fmt::Display;
pub struct People {
    id:i32,
    age:i32,
}
pub struct Animal {
    id:i32,
    age:i32,
}
pub trait Speed {
    fn speed(&self)->i32;
    fn eat(&self) {
        println!("the object  is eating!");
        println!("eating with running,speed is {}",self.speed());
    }
}
impl Speed for People {
    fn speed(&self) ->i32 {
        self.id * self.age + 20
    }
}
impl Speed for Animal {
    fn speed(&self) ->i32 {
        self.id * self.age * 2 + 300
    }
    fn eat(&self) {
        println!("this is age = {} animal is eating!",self.age);
    }
}
pub fn trait_bound1(param:&impl Speed) {
    println!("speed is {}",param.speed());
}
pub fn trait_bound2<T:Speed>(param:&T) {
    println!("bound2: speed is {}",param.speed());
}
pub fn trait_bound3<T:Speed>(param1:&T,param2:&T) {
    println!("bound3:speed1 is {},speed2 is {}",param1.speed(),param2.speed());
}
pub fn trait_bound4<T:Speed + Display>(param:&T) {
    println!("bound3:{}",param);
}
pub fn trait_bound5<T,U,>(param:&T) 
    where T:Speed + Display,
          U:Clone + Display,
{

}
pub fn trait_bound6<T:Speed>(param:&T)->&T {
    return param;
}
pub fn use_trait() {
    let people1 = People {
        id:1,
        age:20,
    };
    let animal1 = Animal {
        id:1,
        age:4,
    };
    let people2 = People {
        id:2,
        ..people1
    };
    println!("people1:speed:{}",people1.speed());
    println!("animal1:speed:{}",animal1.speed());
    people1.eat();
    animal1.eat();
    trait_bound1(&people1);
    trait_bound2(&people1);
    trait_bound3(&people1, &people2);
    // trait_bound4(&people1);
}

