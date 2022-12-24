/// Insertion sort.
///
/// ```rust
/// let mut arr = [2, 6, 3, 4, 9, -3, 4, 1, 5];
/// let sorted_arr = sort::insertion::sort(&mut arr);
/// assert_eq!([-3, 1, 2, 3, 4, 4, 5, 6, 9], sorted_arr);
/// ```
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
