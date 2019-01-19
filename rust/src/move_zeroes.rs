pub fn move_zeroes(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return;
    }

    let mut index = 0;
    for i in 0..nums.len() {
        if nums[i] == 0 && nums[index] != 0 {
            index = i;
        }

        if i > index && nums[index] == 0 && nums[i] != 0 {
            nums[index] = nums[i];
            nums[i] = 0;
            index += 1;
        }
    }
}
