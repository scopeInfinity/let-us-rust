/// Merge sort helper.
///
/// Sorts slice[s..e] using merge sort.
///
fn _merge_sort(slice: &mut[i32], s: usize, e: usize) -> &mut[i32] {
    if e-s <= 1 {
        // <=1 element in slice[s..e], thus it's already sorted.
        return slice;
    }

    // splitting slice into 0...m-1 and m...n-1
    let m = (s+e)/2;
    _merge_sort(slice, s, m);
    _merge_sort(slice, m, e);

    // merge both halves
    let mut i: usize = s;
    let mut j: usize = m;
    let mut k: usize = 0;
    let mut aux = vec![0; e-s];

    while i < m && j < e {
        if slice[i] < slice[j] {
            aux[k] = slice[i];
            i+=1;
        } else {
            aux[k] = slice[j];
            j+=1;
        }
        k+=1;
    }

    while i < m {
        aux[k] = slice[i];
        i+=1;
        k+=1;
    }

    while j < e {
        aux[k] = slice[j];
        j+=1;
        k+=1;
    }

    for k in 0..aux.len() {
        slice[k+s] = aux[k]
    }

    slice
}

/// Merge sort.
///
/// ```rust
/// let mut arr = [2, 6, 3, 4, 9, -3, 4, 1, 5];
/// let sorted_arr = let_us_rust::sort::merge::sort(&mut arr);
/// assert_eq!([-3, 1, 2, 3, 4, 4, 5, 6, 9], sorted_arr);
/// ```
pub fn sort(slice: &mut[i32]) -> &mut[i32] {
    _merge_sort(slice, 0, slice.len())
}
