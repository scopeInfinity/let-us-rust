/// Bubble sort.
///
/// ```rust
/// let mut arr = [2, 6, 3, 4, 9, -3, 4, 1, 5];
/// let sorted_arr = let_us_rust::sort::bubble::sort(&mut arr);
/// assert_eq!([-3, 1, 2, 3, 4, 4, 5, 6, 9], sorted_arr);
/// ```
pub fn sort(slice: &mut[i32]) -> &mut[i32] {
    let n = slice.len();
    for i in 0..n-1 {
        for j in 0..n-1-i {
            if slice[j] > slice[j+1] {
                // swap slice[j] with slice[j+1]
                let temp = slice[j+1];
                slice[j+1] = slice[j];
                slice[j] = temp;
            }
        }
    }
    slice
}
