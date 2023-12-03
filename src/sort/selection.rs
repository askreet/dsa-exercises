fn selection_sort(nums: &mut [i32]) {
    for anchor_index in 0..nums.len() {
        let mut min_index = anchor_index;
        for i in anchor_index..nums.len() {
            if nums[i] < nums[min_index] {
                min_index = i;
            }

            if i == nums.len() {
                break;
            }
        }

        if min_index != anchor_index {
            let s = nums[min_index];
            nums[min_index] = nums[anchor_index];
            nums[anchor_index] = s;
        }
    }
}

#[test]
fn test_selection_sort() {
    let mut nums = [4, 2, 8, 22, 1];

    selection_sort(&mut nums);

    assert_eq!(&[1, 2, 4, 8, 22], &nums);
}
