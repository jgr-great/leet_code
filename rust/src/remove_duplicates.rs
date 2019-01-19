pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    let mut prev = 0; // 上一个值的索引

    let len = nums.len();

    while i < len {
        if nums[prev] != nums[i] && prev != i {
            prev = prev + 1;
            nums[prev] = nums[i];
        }
        i = i + 1;
    }

    if len > 0 {
        prev as i32 + 1
    } else {
        0
    }
}
