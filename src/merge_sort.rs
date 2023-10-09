/// # Example:
///
/// 
/// use base::merge_sort;
/// assert_eq!(merge_sort(&[2,3,5,4,1]), [1,2,3,4,5]);

pub fn merge_sort(vec: &Vec<u32>) -> Vec<u32> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let size = vec.len() / 2;
        let left: Vec<u32>;
        let right;
        (left,right)=rayon::join(||merge_sort(&vec[0..size].to_vec()),||merge_sort(&vec[size..].to_vec()));
        let merged = merge(&left, &right);

        merged
    }
}

fn merge(left: &Vec<u32>, right: &Vec<u32>) -> Vec<u32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<u32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i = i + 1;
        } else {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    merged
}
