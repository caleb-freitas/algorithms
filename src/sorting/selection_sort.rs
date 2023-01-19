pub fn selection_sort<T: Ord>(vec: &mut [T]) {
    for i in 0..vec.len()-1 {
        // assume the `min_index` is the first element index
        let mut min_index = i;

        // test against elements after `i` to find the smallest
        for j in (i + 1)..vec.len() {
            // if this element is less, then it's the new `min_index`
            if vec[j] < vec[min_index] {
                // found new minimum; remember its index
                min_index = j;
            }
        }
        vec.swap(i, min_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec!["d", "a", "c", "b"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        selection_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec!["a"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a"]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec!["a", "b", "c"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c"]);
    }
}
