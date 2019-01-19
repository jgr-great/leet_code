pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let mut k = k as usize % len;

    let mut slice1 = &mut nums[0..len - k];
    slice1.reverse();
    let mut slice2 = &mut nums[len - k..len];
    slice2.reverse();
    nums.reverse();
}
