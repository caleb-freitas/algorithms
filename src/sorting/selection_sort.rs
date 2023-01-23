pub fn selection_sort<T: Ord>(arr: &mut [T]) -> Option<&mut [T]> {
    if arr.len() == 0 {
        return None;
    }

    if arr.len() == 1 {
        return Some(arr);
    }

    for i in 0..arr.len()-1 {
        let mut min_index = i;

        // find the smallest element in `arr[i + 1..n]` and keep track of its index
        for j in (i + 1)..arr.len() {

            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // put the smaller element founded at the index `i` and the element 
        // `arr[i]` at the index `min_index`
        arr.swap(i, min_index);
    }

    return Some(arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        let res = selection_sort(&mut arr);
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5];
        let res = selection_sort(&mut arr);
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_random_sorted() {
        let mut arr = vec![3, 5, 1, 4, 2];
        let expected = vec![1, 2, 3, 4, 5];
        let res = selection_sort(&mut arr);
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_duplicate_elements() {
        let mut arr = vec![3, 5, 1, 5, 4, 2];
        let expected = vec![1, 2, 3, 4, 5, 5];
        let res = selection_sort(&mut arr);
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_edge_case_empty() {
        let mut arr: Vec<i32> = vec![];
        let expected = None;
        let res = selection_sort(&mut arr);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_edge_case_single_element() {
        let mut arr = vec![1];
        let expected = vec![1];
        let res = selection_sort(&mut arr);
        assert_eq!(res.unwrap(), expected);
    }
}
