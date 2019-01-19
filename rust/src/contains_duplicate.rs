pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return false;
    }

    let mut nums = nums;
    nums.sort();

    for i in 0..(nums.len() - 1) {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }
    false
}
