pub fn linear_search<T: PartialEq>(vec: &[T], target: T) -> Option<usize> {
    for (item_index, item) in vec.iter().enumerate() {
        if *item == target {
            return Some(item_index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_target_exist() {
        let arr = [1, 2, 3, 4, 5];
        let target = 3;
        let index = linear_search(&arr, target);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn test_linear_search_target_do_not_exist() {
        let arr = [1, 2, 3, 4, 5];
        let target = 9;
        let index = linear_search(&arr, target);
        assert_eq!(index, None);
    }
}

