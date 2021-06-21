pub struct Animal<T> {
    weight:T,
    height:T,
}
pub enum Type<K>{
    PIG(K),
    DOG(K),
}
#[derive(Debug)]
pub struct Complex<K,V> {
    k:K,
    v:V,
}
impl<K,V> Complex<K,V> {
    pub fn com<S,T>(self,other:Complex<S,T>) -> Complex<K,T>{
        // println!("self:k:{:?},other:v:{:?}",self.k,com.v);
        Complex {
            k:self.k,
            v:other.v,
        }
    }
}
pub fn use_generic() {
    let vec1 = vec![1,2,3,4,5,6]; 
    println!("max:{}",max(&vec1));
    println!("max2:{}",max_arr(&vec1));
    println!("vec1:{:?}",vec1);
    let vec2 = vec!['a','b','c','d'];
    println!("max_char:{}",max_char(&vec2));
    println!("vec2:{:?}",vec2);
    println!("max_int:{}",max_generic(&vec1));
    println!("max_char:{}",max_generic(&vec2));
    println!("max_int2:{}",max_generic2(&vec1));
    println!("max_char2:{}",max_generic2(&vec2));
    let animal1 = Animal{
        weight:100,
        height:10001,
    };
    let pig = Type::PIG(200);
    let dog = Type::DOG(100.23);
    let result:Result<i32,String> = Result::Ok(11);
    let comp1 = Complex {
        k:32,
        v:32.0,
    };
    let comp2 = Complex {
        k:String::from("yyy"),
        v:'q',
    };
    
    let comp3 = comp1.com(comp2); 
    println!("comp3:{:?}",comp3);

}
pub fn max(vec:&Vec<i32>)->&i32 {
   let mut max = &vec[0];
   for i in vec {
        if max < i {
           max = i; 
        }
   }
   max
}

pub fn max_arr(vec:&[i32])-> i32{
    let mut max = vec[0];
    for &i in vec {
        if max < i {
            max = i;
        }
    }
    max
}

pub fn max_char(vec:&[char])->char {
    let mut max = vec[0];
    for &i in vec{
        if max < i {
            max = i;
        }
    }
    max
}
pub fn max_generic<T:PartialOrd + Copy>(vec:&[T])->T {
    let mut max = vec[0];
        for &i in vec{
            if max < i {
                max = i;
            }
        }
        max
    }
pub fn max_generic2<T:PartialOrd >(vec:&[T])->&T {
    let mut max = &vec[0];
        for i in vec{
            if max < i {
                max = i;
            }
        }
        max
    }
