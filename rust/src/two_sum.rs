pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut buf = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        if let Some(&t) = buf.get(&(target - v)) {
            if t != i {
                return vec![i as i32, t as i32];
            }
        }
        buf.insert(v, i);
    }
    vec![]
}
