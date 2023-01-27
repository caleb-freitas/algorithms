// pub fn bottom_up_merge_sort<T: PartialOrd>(arr: &mut [T], p: usize, q: usize, r: usize) -> &mut [T] {
//     let left_size = q - p + 1;
//     let right_size = r - q;
//
//     let mut left = Vec::with_capacity(left_size);
//     let mut right = Vec::with_capacity(right_size);
//
//     for i in 0..left_size {
//         left[i] = arr[p + i];
//     }
//
//     for j in 0..right_size {
//         right[j] = arr[q + j + 1];
//     }
//
//     let mut left_index = 0;
//     let mut right_index = 0;
//     let mut a_index = p;
//
//     while left_index < left_size && right_index < right_size {
//         if left[left_index] <= right[right_index] {
//             arr[a_index] = left[left_index];
//             left_index += 1;
//         } else {
//             arr[a_index] = right[right_index];
//             right_index += 1;
//         }
//         a_index += 1;
//     }
//
//     while left_index < left_size {
//         arr[a_index] = left[left_index];
//         left_index += 1;
//         a_index += 1;
//     }
//
//     while right_index < right_size {
//         arr[a_index] = right[right_index];
//         right_index += 1;
//         a_index += 1;
//     }
//
//     return arr;
// }
