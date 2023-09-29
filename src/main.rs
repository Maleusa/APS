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

    // into couples
    let v = vec![0, 1, 2, 3];
    assert_eq!(lib::into_couples(&v), vec![(0, 1), (2, 3)]);


    // sum_even_n
    let t = [3, 1, 5, 2, 3, 2, 6, 4, 3, 7, 4];
    assert_eq!(lib::sum_even_n(&t, 1), 2);
    assert_eq!(lib::sum_even_n(&t, 2), 4);
    assert_eq!(lib::sum_even_n(&t, 3), 10);
}
