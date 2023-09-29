fn main() {
    use base::dot_product;
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    assert_eq!(dot_product(&a1, &a2), 1 * 4 + 2 * 5 + 3 * 6);
    println!("dot_product DONE");

    use base::is_sorted;
    assert!(is_sorted(&[1, 2, 3, 4]));
    assert!(!is_sorted(&[1, 3, 2, 4]));
    assert!(!is_sorted(&[1, 2, 4, 3]));
    println!("is_sorted DONE");

    use base::into_couples;
    let v = vec![0, 1, 2, 3];
    // println!("v = {:?}", into_couples(&v));
    assert_eq!(into_couples(&v), vec![(0, 1), (2, 3)]);
    println!("into_couples DONE");

    use base::sum_even_n;
    let t = [3, 1, 5, 2, 3, 2, 6, 4, 3];
    assert_eq!(sum_even_n(&t, 1), 2);
    assert_eq!(sum_even_n(&t, 2), 4);
    assert_eq!(sum_even_n(&t, 3), 10);
    println!("sum_even_n DONE");

    use base::number_of_a;
    let string_slice = "a vous dirais-je maman";
    assert_eq!(number_of_a(string_slice), 4);
    println!("number_of_a DONE");

    use base::number_of_01;
    assert_eq!(number_of_01(&[0, 1, 1, 1, 4, 0, 1, 0, 2]), (3, 4));
    assert_eq!(number_of_01(&[2, 5, 1, 1, 2]), (0, 2));
    println!("number_of_01 DONE");
}
