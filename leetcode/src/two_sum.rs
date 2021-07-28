use std::collections::HashMap;
pub fn exec(nums :Vec<i32>, target: i32)-> Vec<i32> {
    let mut map = HashMap::new();
    for (idx,&num) in nums.iter().enumerate() {
        let opt = map.get(&(target - num));
        if let Some(&i) = opt {
            return vec![i as i32,idx as i32];
        }
        map.insert(num, idx);
    }
    panic!()

}
