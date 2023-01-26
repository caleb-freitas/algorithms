pub fn insertion_sort(a: &mut [i32]) -> &mut [i32]{
    for current in 1..a.len() {
        // assume that key is the current element
        let key = a[current];

        // assume that the previous index is the current index minus 1
        let mut previous = current - 1;

        // check where in the sorted array to put the current element
        while a[previous] > key {
            a[previous + 1] = a[previous];
            previous -= 1;
        }
        a[previous + 1] = key;
    }
    return a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_random_sorted() {
        let mut arr = vec![3, 5, 1, 4, 2];
        let expected = vec![1, 2, 3, 4, 5];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_duplicate_elements() {
        let mut arr = vec![3, 5, 1, 5, 4, 2];
        let expected = vec![1, 2, 3, 4, 5, 5];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_edge_case_empty() {
        let mut arr: Vec<i32> = vec![];
        let expected = vec![];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_edge_case_single_element() {
        let mut arr = vec![1];
        let expected = vec![1];
        let res = insertion_sort(&mut arr);
        assert_eq!(res, expected);
    }
}
