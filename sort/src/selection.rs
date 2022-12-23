pub fn sort(slice: &mut[i32]) -> &mut[i32] {
    let n = slice.len();
    for i in 0..n-1 {
        let mut low_index = i;
        for j in i+1..n {
            if slice[j] < slice[low_index] {
                low_index = j;
            }
        }
        // swap slice[i] with smallest number in slice[i..n]
        let temp = slice[low_index];
        slice[low_index] = slice[i];
        slice[i] = temp;
    }
    slice
}