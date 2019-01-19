pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    if digits.is_empty() {
        return vec![];
    }
    let mut digits = digits;
    let len = digits.len();
    let mut tmp = vec![];

    for i in (0..len).rev() {
        if i != 0 && digits[i] == 9 {
            digits[i] = 0;
            continue;
        }

        if i == 0 && digits[i] == 9 {
            digits[i] = 0;
            tmp.push(1);
            tmp.append(&mut digits);
            break;
        }
        digits[i] = digits[i] + 1;
        break;
    }

    if !tmp.is_empty() {
        tmp
    } else {
        digits
    }
}
