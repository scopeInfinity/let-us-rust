pub fn sort(slice: &mut[i32]) -> &mut[i32] {
    let n = slice.len();
    for sorted_len in 1..n {
        let mut index = sorted_len;
        let new_element = slice[index];
        while index>=1 && slice[index-1] > new_element {
            slice[index] = slice[index-1];
            index -= 1;
        }
        slice[index] = new_element;
    }
    slice
}