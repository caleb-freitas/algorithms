// Using the divide and conquer method, this function returns a tuple containing
// the indices that demarcate a maximum subarray, along with the sum of the values
// in a maximum subarray.
pub fn divide_conquer_find_maximum_subarray(arr: &[i32]) -> (i32, usize, usize) {
    // Get array length.
    let n = arr.len();

    // Base case -> only one item in the array.
    if n == 1 {
        return (arr[0], 0, 0);
    }

    // Get the mid index (integer division -> 3 / 2 = 1 ).
    let mid = n / 2;
    
    // Conquer by recursively finding maximum subarrays within the left and right subarrays.
    let (left_sum, left_low, left_high) = divide_conquer_find_maximum_subarray(&arr[..mid]);
    let (right_sum, right_low, right_high) = divide_conquer_find_maximum_subarray(&arr[mid..]);

    // Find a maximum subarray that crosses the midpoint.
    let (cross_sum, cross_low, cross_high) = find_maximum_crossing_subarray(arr, mid);

    // Any contiguous subarray `sub_arr[i..j]` of a `arr[low..high]` must lie
    // in exactly one of the following places: 
    // 1. Entirely in the subarray `arr[low..mid`.
    // 2. Entirely in the subarray `arr[mid + 1..high]`.
    // 3. Crossing the midpoint.
    // Thus, this if statement check where the contiguous subarray is. It's the "combine" step.
    if left_sum >= right_sum && left_sum >= cross_sum {
        return (left_sum, left_low, left_high);
    } else if right_sum >= left_sum && right_sum >= cross_sum {
        return (right_sum, right_low + mid, right_high + mid);
    } else {
        return (cross_sum, cross_low, cross_high);
    }
}

// Returns a tuple containing the indices demarcating a maximum subarray that
// crosses the midpoint, along with the sum of the values in a maximum array.
//
// It's a complementary function for `divide_conquer_find_maximum_subarray`.
fn find_maximum_crossing_subarray(arr: &[i32], mid: usize) -> (i32, usize, usize) {
    // Holds the greatest sum found in the left half
    let (mut left_sum, mut left_index) = (i32::min_value(), mid);
    let mut sum = 0;
    for i in (0..mid).rev() {
        sum += arr[i];
        if sum > left_sum {
            left_sum = sum;
            left_index = i;
        }
    }

    // Holds the greatest sum found in the right half.
    let (mut right_sum, mut right_index) = (i32::min_value(), mid);
    sum = 0;
    for i in mid..arr.len() {
        sum += arr[i];
        if sum > right_sum {
            right_sum = sum;
            right_index = i;
        }
    }

    (left_sum + right_sum, left_index, right_index)
}

// This function use an "trick", that is the following: when the `current_sum`
// becomes negative, it means that all the previous elements in the subarray have
// a sum less than zero, and including any more elements from that subarray would
// further decrease the sum instead of increasing it.
//
// In this case, it is more beneficial to start a new subarray from the current
// position, and discard the previous subarray with the negative sum.
pub fn linear_find_maximum_subarray(arr: &[i32]) -> (usize, usize, i32) {
    let mut max_sum = std::i32::MIN;
    let mut left_index = 0;
    let mut right_index = 0;
    let mut current_sum = 0;

    for i in 0..arr.len() {
        // Keep track of the maximum sum seen so far.
        current_sum += arr[i];
        if current_sum > max_sum {
            max_sum = current_sum;
            right_index = i;
        }
        
        // Do the trick: start a new array starting from `i + 1` (the next item),
        // and discard the previous subarray.
        if current_sum < 0 {
            current_sum = 0;
            left_index = i + 1;
        }
    }

    (left_index, right_index, max_sum)
}

// This function has the same signature as the others, but in this case,
// the implementation compare every pair in the array, thus having a runnig time 
// of O(n^2).
pub fn brute_force_find_maximum_subarray(arr: &[i32]) -> (usize, usize, i32) {
    let mut max_sum = std::i32::MIN;
    let mut left_index = 0;
    let mut right_index = 0;

    // Iterates through all possible starting indices of the subarray.
    for i in 0..arr.len() {
        let mut current_sum = 0;

        // Iterates through all possible ending indices of the subarray,
        // starting from the current starting index.
        for j in i..arr.len() {

            // Calculate the current sum of the subarray between the indices
            // i and j by adding the current element to current_sum and compare
            // it to the current maximum sum.
            current_sum += arr[j];
            if current_sum > max_sum {
                max_sum = current_sum;
                left_index = i;
                right_index = j;
            }
        }
    }

    (left_index, right_index, max_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brute_force_max_subarray() {
        let arr = [-2, 1, -3, 4, -1, 2, 1, -5];
        let (start, end, sum) = brute_force_find_maximum_subarray(&arr);
        assert_eq!(start, 3);
        assert_eq!(end, 6);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_linear_find_maximum_subarray() {
        let arr = [-2, 1, -3, 4, -1, 2, 1, -5];
        let (start, end, sum) = linear_find_maximum_subarray(&arr);
        assert_eq!(start, 3);
        assert_eq!(end, 6);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_max_subarray_crossing_midpoint() {
        let arr = [-2, 1, -3, 4, -1, 2, 1, -5];
        let (sum, start, end) = divide_conquer_find_maximum_subarray(&arr);
        assert_eq!(end, 6);
        assert_eq!(start, 3);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_max_subarray_left_half() {
        let arr = [1, 1, 1, 1, 0, 0, 0, 0, 0];
        let (sum, start, end) = divide_conquer_find_maximum_subarray(&arr);
        assert_eq!(end, 3);
        assert_eq!(start, 0);
        assert_eq!(sum, 4);
    }

    #[test]
    fn test_max_subarray_right_half() {
        let arr = [0, 0, 0, 0, 0, 1, 1, 1, 1];
        let (sum, start, end) = divide_conquer_find_maximum_subarray(&arr);
        assert_eq!(start, 5);
        assert_eq!(end, 8);
        assert_eq!(sum, 4);
    }

    #[test]
    fn test_max_subarray_with_one_element() {
        let arr = [1];
        let (sum, start, end) = divide_conquer_find_maximum_subarray(&arr);
        assert_eq!(end, 0);
        assert_eq!(start, 0);
        assert_eq!(sum, 1);
    }
}
