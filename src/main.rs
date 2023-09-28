use base::dot_product;

fn main() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    println!("Hello world");
    println!(" res = {:?} ", dot_product(&a1, &a2));

    use base::is_sorted;
    assert!(is_sorted(&[1, 2, 3, 4]));
    assert!(!is_sorted(&[1, 3, 2, 4]));
    assert!(!is_sorted(&[1, 2, 4, 3]));

    use base::into_couples;
    let v = vec![0, 1, 2, 3];
    println!("v = {:?}", into_couples(&v));
    assert_eq!(into_couples(&v), vec![(0, 1), (2, 3)])
}
