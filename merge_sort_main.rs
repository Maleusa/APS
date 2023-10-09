use std::num::Wrapping;

const N: usize = 100;

fn main() {
    let mut v: Vec<u64> = std::iter::repeat_with(rand::random).take(N).collect();
    let checksum: Wrapping<u64> = v.iter().copied().map(Wrapping).sum();

    merge_sort(&mut v);
}

fn merge_sort(v: &[u64]) -> _ {
    merge_sort_rec(v);
}

fn merge_sort_rec(v: &[u64]) -> &[u64] {
    // base case
    if v.len() <= 1 {
        return v;
    }

    let (v1, v2) = v.split_at(v.len() / 2);
    merge_fusion(merge_sort_rec(v1), merge_sort_rec(v2))
}

fn merge_fusion(v1: &[u64], v2: &[u64]) -> Vec<u64> {
    let mut v_res: Vec<u64> = Vec::new();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut v: u64;
    // push the smallest values until one slice is empty
    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
            v = v1[i];
            i += 1;
        } else {
            v = v2[j];
            j += 1;
        }
        v_res.push(v);
    }
    // push the rest of the values
    while i < v1.len(){
        v_res.push(v1[i]);
            i += 1;
    }
    while j < v2.len(){
        v_res.push(v2[j]);
            j += 1;
    }
    v_res
}
