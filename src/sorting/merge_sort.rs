pub fn merge_sort(arr: &mut [i32], p: usize, q: usize, r: usize) {
   let left_size = q - p + 1;
    let right_size = r - q;

    let mut left = vec![0; left_size];
    let mut right = vec![0; right_size];

    for i in 0..left_size {
        left[i] = arr[p + i];
    }

    for j in 0..right_size {
        right[j] = arr[q + j + 1];
    }

    let mut left_index = 0;
    let mut right_index = 0;
    let mut a_index = p;

    while left_index < left_size && right_index < right_size {
        if left[left_index] <= right[right_index] {
            arr[a_index] = left[left_index];
            left_index += 1;
        } else {
            arr[a_index] = right[right_index];
            right_index += 1;
        }
        a_index += 1;
    }

    while left_index < left_size {
        arr[a_index] = left[left_index];
        left_index += 1;
        a_index += 1;
    }

    while right_index < right_size {
        arr[a_index] = right[right_index];
        right_index += 1;
        a_index += 1;
    }
}
