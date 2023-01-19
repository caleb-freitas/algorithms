pub fn insertion_sort(a: &mut [i32]) {
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
}
