// mod include the whole module
mod lib;

fn main() {
    println!("Hello, wolrd!");
    lib::say_hi();
    // dot product
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    assert_eq!(lib::dot_product(&a1, &a2), 1 * 4 + 2 * 5 + 3 * 6);

    // is sorted
    assert!(lib::is_sorted(&[1,2,3,4]));
    assert!(!lib::is_sorted(&[1,3,2,4]));
    assert!(!lib::is_sorted(&[1,2,4,3]));
}
