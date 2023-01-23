pub fn linear_search<T: PartialEq>(arr: &[T], target: T) -> Option<usize> {

    if arr.len() == 0 {
        return None;
    }

    for (index, item) in arr.iter().enumerate() {
        if *item == target {
            return Some(index);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_exist() {
        let arr = [1, 2, 3, 4, 5];
        let target = 3;
        let index = linear_search(&arr, target);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn test_target_dont_exist() {
        let arr = [1, 2, 3, 4, 5];
        let target = 9;
        let index = linear_search(&arr, target);
        assert_eq!(index, None);
    }

    #[test]
    fn test_edge_case_empty() {
        let arr: Vec<i32> = vec![];
        let target = 0;
        let expected = None;
        let res = linear_search(&arr, target);
        assert_eq!(res, expected);
    }
}
