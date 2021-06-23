#[cfg(test)]
pub mod tests{
    use crate::*;

    #[test]
    pub fn test_success() {
        println!("test");
    }
    #[test]
    pub fn test_fail() {
        panic!("test fail");
    }
    #[test]
    pub fn test_assert(){
        let a = 5;
        let b = 6;
        assert!(a < b);
        assert!(a > b);
    }
    #[test]
    pub fn test_assert_eq() {
        let a = 5;
        let b = 5;
        assert_eq!(a , b);
        let b = 6;
        assert_eq!(a , b);
                
    }
    #[test]
    pub fn test_assert_ne() {
        let a = 5;
        let b = 6;
        assert_ne!(a,b);
        let b = 5;
        assert_ne!(a,b);
    }
    #[test]
    pub fn test_assert_info() {
        let a = 5;
        assert!(a > 7,"a > 7:{}",false);
    }
    #[test]
    #[should_panic]
    pub fn test_panic() {
        panic!("test panic");
    }
    #[test]
    #[should_panic(expected =  "test panic match :")]
    pub fn test_panic_match() {
        // panic!("test panic match :{}",true);
        // panic!("test panic match :{}",false);
        panic!("test panic match224 :{}",false);
        
    }
    #[test]
    pub fn test_other_mod() {
        vec::use_vector();
    }

    #[test]
    #[ignore]
    pub fn test_use_result1() -> Result<(),String>{
        let a = 5;
        if a > 4 {
            return Ok(());
        }
        Err(String::from("result --- error"))
    }
    #[test]
    pub fn test_use_result2() -> Result<(),String>{
        let a = 5;
        if a > 6 {
            return Ok(());
        }
        Err(String::from("result --- error"))
    }
}
