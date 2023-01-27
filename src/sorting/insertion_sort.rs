/// Sorts a slice in-place.
/// Time complexity: O(n^2).
/// Space complexity: O(1).
pub fn insertion_sort<T: PartialOrd>(s: &mut [T]) -> &mut [T]{
    // iterate over each element of the slice s[1..n], where n is the length
    for i in 1..s.len() {
        let mut j = i;

        // move elements of s[0..i-1] that are greater than s[i], to one
        // position to the right from their current position
        while j > 0 && s[j - 1] > s[j] {
            s.swap(j - 1, j);
            j -= 1;
        }
    }
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn random_sorted() {
        let mut arr = vec![3, 5, 1, 4, 2];
        let expected = vec![1, 2, 3, 4, 5];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn duplicate_elements() {
        let mut arr = vec![3, 5, 1, 5, 4, 2];
        let expected = vec![1, 2, 3, 4, 5, 5];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn edge_case_empty() {
        let mut arr: Vec<i32> = vec![];
        let expected = vec![];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn edge_case_single_element() {
        let mut arr = vec![1];
        let expected = vec![1];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }
}
