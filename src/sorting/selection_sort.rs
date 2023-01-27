/// Sort a slice in-place.
/// Time complexity: O(n^2).
/// Space complexity: O(1).
pub fn selection_sort<T: PartialOrd>(s: &mut [T]) -> &mut [T] {
    for i in 0..s.len() {
        let mut min_index = i;

        // find the smallest element in `s[i + 1..n]` and keep track of its index
        for j in (i + 1)..s.len() {

            if s[j] < s[min_index] {
                min_index = j;
            }
        }

        // put the smaller element founded at the index `i` and the element 
        // `s[i]` at the index `min_index`
        s.swap(i, min_index);
    }

    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_sorted() {
        let mut s = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        let res = selection_sort(&mut s);
        assert_eq!(res, expected);
    }

    #[test]
    fn reverse_sorted() {
        let mut s = vec![5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5];
        let res = selection_sort(&mut s);
        assert_eq!(res, expected);
    }

    #[test]
    fn random_sorted() {
        let mut s = vec![3, 5, 1, 4, 2];
        let expected = vec![1, 2, 3, 4, 5];
        let res = selection_sort(&mut s);
        assert_eq!(res, expected);
    }

    #[test]
    fn duplicate_elements() {
        let mut s = vec![3, 5, 1, 5, 4, 2];
        let expected = vec![1, 2, 3, 4, 5, 5];
        let res = selection_sort(&mut s);
        assert_eq!(res, expected);
    }

    #[test]
    fn edge_case_empty() {
        let mut s: Vec<i32> = vec![];
        let expected = vec![];
        let res = selection_sort(&mut s);
        assert_eq!(res, expected);
    }

    #[test]
    fn edge_case_single_element() {
        let mut s = vec![1];
        let expected = vec![1];
        let res = selection_sort(&mut s);
        assert_eq!(res, expected);
    }
}
