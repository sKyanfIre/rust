#[cfg(test)]
mod tests {
    use super::front_of_house::Breakfast;
    use super::front_of_house;
    use rand::prelude::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn eat_breakfast() {
        let breakfast = Breakfast::new(1,String::from("bread"),29.1);
        breakfast.eat();
        front_of_house::owner();

    }
    #[test]
    fn rand_function() {
        let mut rng = rand::thread_rng();
        for i in 1..100 {
            let y: f32 = rng.gen();
            println!("rand:{}",y);
        }
    }
}
mod front_of_house {
    pub fn owner() {
        println!("owner------");
    }
    #[derive(Debug)]
    pub struct Breakfast {
        id:i32,
        name:String,
        price:f32,
    }
    impl Breakfast {
        pub fn new(id:i32,name:String,price:f32)-> Breakfast {
            Breakfast{
                id,
                name,
                price,
            }
        }
        pub fn eat(&self) {
            println!("eat Breakfast:{:?}",self);
        }
    }
    pub mod hosting {
        pub fn add_to_waitlist(){}
        pub fn seat_to_table(){}
    }
    pub mod serving {
        fn take_order() {}
        fn server_order(){
            // super::hosting::seat_to_table();
            take_order();
            super::hosting::seat_to_table();

        }
        pub fn take_payment(){}
    }
    fn test(){
        hosting::seat_to_table();
    }
    
}
pub fn eat_at_rest(){
    // absolute
    crate::front_of_house::hosting::add_to_waitlist();
    // relative
    front_of_house::serving::take_payment();
}
