pub fn single_number(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }
    let mut nums = nums;
    nums.sort();
    let len = nums.len();
    for i in 0..len {
        if i >= 1 && i < len - 1 && nums[i] != nums[i - 1] && nums[i] != nums[i + 1] {
            return nums[i];
        }

        if i < 1 && nums[i] != nums[i + 1] {
            return nums[i];
        }

        if i == len - 1 && nums[i] != nums[i - 1] {
            return nums[i];
        }
    }
    0
}
