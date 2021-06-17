// common struct
#[derive(Debug)]
struct People {
    id:i32,
    username:String,
    code:String,
    height:f32,
    weight:f32,
    age:i8,
    male:bool
}
// struct method
impl People {
    fn cal(&self){
       println!("height * weight = {}",self.height*self.weight); 
    }
    fn view(&self,view:&str) {
        println!("{} see a boy named {}",view,self.username);
    }
    fn update_name(&mut self,name:String){
        self.username = name;
    }
    fn new(id:i32,username:String,code:String,height:f32,weight:f32,age:i8,male:bool)-> People {
        People{
            id,
            username,
            code,
            height,
            weight,
            age,
            male
        }
    }
}
// tuple struct
#[derive(Debug)]
struct Base(i32,f32,f32,i8);
fn main(){
    let people1 = People{
        id:1,
        username:String::from("zzz"),
        code:String::from("code1"),
        height: 175.89,
        weight: 65.2,
        age:20,
        male:true,
    }; 

    println!("people1:{:#?}",people1);
    let people2 = People{
        id:2,
        username:String::from("zzz"),
        code:String::from("code2"),
        ..people1
    }; 
    println!("people2:{:#?}",people2);
    let mut people3 = build_people(3,String::from("yyy"),String::from("code3"),120.9,59.1,20,false);
    println!("people3:{:#?}",people3);
    people3.username =String::from(  "xxx" );
    println!("people3:{:#?}",people3);
    let base1 = Base(people3.id,people3.height,people3.weight,people3.age);
    println!("base1:{:?}",base1);
    println!("base1:0:{},1:{},2:{},3:{}",base1.0,base1.1,base1.2,base1.3);
    cal_people(people3);
    // println!("people3:{:?}",people3);
    
    let mut people1 = people1;
    people1.cal();
    people1.update_name(String::from( "yevu" ));
    people1.view("yyy");
    let people4 = People::new(5,String::from( "111" ),String::from( "code5" ),20.1,21.1,11,true);
    println!("people4:{:?}",people4);
}
fn build_people(id:i32,username:String,code:String,height:f32,weight:f32,age:i8,male:bool)->People{
    People{
        id,
        username,
        code,
        height,
        weight,
        age,
        male
    }
}

fn cal_people(people:People) {
    println!("weight*height:{}",people.weight * people.height);
}
