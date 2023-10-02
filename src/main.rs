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

    use base::first_place;
    let t = [3, 1, 5, 2, 7, 6];
    assert_eq!(first_place(&t, 5), Some(2));
    assert_eq!(first_place(&t, 3), Some(0));
    assert_eq!(first_place(&t, 8), None);
    println!("first_place DONE");

    use base::digits;
    assert_eq!(digits(&["5 * 5", "=25"]), String::from("5525"));
    assert_eq!(digits(&["1+2", "*3", "=9"]), String::from("1239"));
    println!("digits DONE");

    use base::digits_sum;
    assert_eq!(digits_sum(123), 6);
    assert_eq!(digits_sum(456), 15);
    println!("digits_sum DONE");

    use base::enough_sum;
    assert_eq!(enough_sum(|i| 1.0, 9.5), 10);
    assert_eq!(enough_sum(|i| 1.0 / (i as f64), 2.0), 4); // 1+1/2+1/3+1/4
    println!("enough_sum DONE");

    use base::count_extremum;
    assert_eq!(count_extremum(&[]), 0);
    assert_eq!(count_extremum(&[0]), 1);
    assert_eq!(count_extremum(&[0, 0]), 1);
    assert_eq!(count_extremum(&[0, 1, 0]), 3);
    assert_eq!(count_extremum(&[0, 1, 2, 3, 4, 4, 4, 3, 2, 1, 0]), 3);
    assert_eq!(count_extremum(&[0, 1, 2, 2, 1, 1, 2, 3, 2]), 5);
    println!("count_extremum DONE");
}
