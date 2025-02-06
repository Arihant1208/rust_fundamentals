use std::collections::HashMap;


fn combinations (n : i32,r : i32) -> i32 {
    let mut ans = 1;
    for i in 0..r{
        ans *= (n-i);
    }
    return ans 
}

pub fn solve() -> i32{
    let mut hm = HashMap::new();
    let mut ans = 0;

    let mut nums :Vec<i32> = vec![1,2,4,5,10];


    for i in 0..nums.len(){
        for j in (i+1)..nums.len() {
            let count = hm.entry(nums[i]*nums[j]).or_insert(0);
            *count += 1;
        }
    }
    for val in hm.values() {
        if *val >= 2 {
            ans += combinations(*val, 2);
        }
    }
    return ans*4;
}
