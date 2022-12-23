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