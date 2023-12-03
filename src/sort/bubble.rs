pub fn bubble_sort(nums: &mut [i32]) {
    let mut sort_until = nums.len();
    if nums.len() < 2 {
        return;
    }

    loop {
        let mut changed = false;
        let mut i = 0;
        let mut j = 1;

        loop {
            if nums[i] > nums[j] {
                let s = nums[i];
                nums[i] = nums[j];
                nums[j] = s;
                changed = true
            }

            i += 1;
            j += 1;
            if j >= sort_until {
                break;
            }
        }

        match changed {
            true => sort_until -= 1,
            false => break,
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut nums = [4, 2, 8, 22, 1];

    bubble_sort(&mut nums);

    assert_eq!(&[1, 2, 4, 8, 22], &nums);
}
