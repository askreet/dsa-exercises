fn insertion_sort(nums: &mut [i32]) {
    for temp_index in 1..nums.len() {
        let temp = nums[temp_index];
        let mut comparison_index = temp_index;
        loop {
            if nums[comparison_index - 1] > temp {
                nums[comparison_index] = nums[comparison_index - 1]
            } else {
                break;
            }

            comparison_index -= 1;
            if comparison_index == 0 {
                break;
            }
        }
        nums[comparison_index] = temp;
    }
}


#[test]
fn test_insertion_sort() {
    let mut nums = [4, 2, 8, 22, 1];

    insertion_sort(&mut nums);

    assert_eq!(&[1, 2, 4, 8, 22], &nums);
}
