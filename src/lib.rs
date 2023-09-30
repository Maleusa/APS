use itertools::Itertools;

pub fn say_hi() -> bool {
    println!("Hi from fonction");
    return true;
}
/// Return the dot product of s1 by s2.
///
/// # Example:
///
/// ```
/// use base::dot_product;
/// let a1 = [1, 2, 3];
/// let a2 = [4, 5, 6];
/// assert_eq!(dot_product(&a1, &a2), 1*4+2*5+3*6)
/// ```
pub fn dot_product(s1: &[u32], s2: &[u32]) -> u32 {
    let iter = std::iter::zip(s1, s2);
    iter.map(|(a, b)| *a * *b).sum()
}

/// Return if given slice is sorted.
///
/// # Example:
///
/// ```
/// use base::is_sorted;
/// assert!(is_sorted(&[1,2,3,4]));
/// assert!(!is_sorted(&[1,3,2,4]));
/// assert!(!is_sorted(&[1,2,4,3]));
/// ```
pub fn is_sorted(slice: &[u32]) -> bool {
    slice.windows(2).all(|v| v[0] <= v[1])
}

/// Transform a slice n u32 (n even) into a vector of n/2 couples of u32.
///
/// # Example:
///
/// ```
/// use base::into_couples;
/// let v = vec![0, 1, 2, 3];
/// assert_eq!(into_couples(&v), vec![(0, 1), (2, 3)])
/// ```
pub fn into_couples(s: &[u32]) -> Vec<(u32, u32)> {
    let iter = s.chunks(2);
    iter.map(|v| (v[0], v[1])).collect()
}

/// Return the sum of the n first even integers of the given slice.
/// If not enough sum all of them.
///
/// # Example:
/// ```
/// use base::sum_even_n;
/// let t = [3, 1, 5, 2, 3, 2, 6, 4, 3, 7, 4];
/// assert_eq!(sum_even_n(&t, 1), 2);
/// assert_eq!(sum_even_n(&t, 2), 4);
/// assert_eq!(sum_even_n(&t, 3), 10);
/// ```
pub fn sum_even_n(slice: &[u32], n: usize) -> u32 {
    let (even, _): (Vec<_>, Vec<_>) = slice.iter().partition(|x| *x % 2 == 0);
    even.iter().take(n).sum()
}

/// Return the number of 'a' in given string slice.
///
/// # Example:
///
/// ```
/// use base::number_of_a;
/// let string_slice = "a vous dirais-je maman";
/// assert_eq!(number_of_a(string_slice), 4)
/// ```
pub fn number_of_a(s: &str) -> usize {
    s.chars().filter(|c| *c == 'a').count()
}

/// Return the number of 0 and the number of 1 in given slice.
///
/// # Example
///
/// ```
/// use base::number_of_01;
/// assert_eq!(number_of_01(&[0, 1, 1, 1, 4, 0, 1, 0, 2]), (3, 4));
/// assert_eq!(number_of_01(&[2, 5, 1, 1, 2]), (0, 2));
/// ```
pub fn number_of_01(slice: &[u32]) -> (usize, usize) {
    
    //let (z, other):(Vec<_>, Vec<_>) = slice.into_iter().partition(|&x| *x == 0u32);
    //let (zeros, other):(Vec<_>, Vec<_>) = slice.iter().partition(|&x| *x == 0u32);
    //let (ones, _):(Vec<_>, Vec<_>) = other.iter().partition(|&x| *x == 0u32);
    //(zeros.len(), ones.len())

    slice.iter().fold((0, 0), |(z, o), &e| {
        if e == 0 {
            return (z + 1, o);
        }
        if e == 1 {
            return (z, o + 1);
        }
        else {
            return (z, o);
        }
    })
}

/// Return the position of the first occurence of given number in given slice.
///
/// # Example:
/// ```
/// use base::first_place;
/// let t = [3, 1, 5, 2, 7 ,6];
/// assert_eq!(first_place(&t, 5), Some(2));
/// assert_eq!(first_place(&t, 3), Some(0));
/// assert_eq!(first_place(&t, 8), None);
/// ```
pub fn first_place(slice: &[u32], target: u32) -> Option<usize> {
    unimplemented!()
}

/// Return the String formed by all digits in given strings.
///
/// # Example:
///
/// ```
/// use base::digits;
/// assert_eq!(digits(&["5 * 5", "=25"]), String::from("5525"));
/// assert_eq!(digits(&["1+2", "*3", "=9"]), String::from("1239"));
/// ```
pub fn digits(strings: &[&str]) -> String {
    unimplemented!()
}

/// Return the sum of digits of given integer.
///
/// # Example:
///
/// ```
/// use base::digits_sum;
/// assert_eq!(digits_sum(123), 6);
/// assert_eq!(digits_sum(456), 15);
/// ```
pub fn digits_sum(integer: u32) -> u32 {
    unimplemented!()
}

/// Return for which i the sum of f(i) (from 1 to infinity) is greater than n.
///
/// # Example:
/// ```
/// use base::enough_sum;
/// assert_eq!(enough_sum(|i| 1.0, 9.5), 10);
/// assert_eq!(enough_sum(|i| 1.0/(i as f64), 2.0), 4); // 1+1/2+1/3+1/4
/// ```
pub fn enough_sum<F>(f: F, limit: f64) -> usize
where
    F: Fn(usize) -> f64,
{
    unimplemented!()
}

/// Return the vector composed of start integer then all
/// intermediate integers then end integer.
///
/// # Example:
/// ```
/// use base::intervals;
/// assert_eq!(intervals(0, 4, 1..4), vec![0, 1, 2, 3, 4]);
/// ```
pub fn intervals<I>(start: u32, end: u32, intermediate: I) -> Vec<u32>
where
    I: IntoIterator<Item = u32>,
{
    unimplemented!()
}

/// Return how many local extremum points (local min or local max) are
/// present in the slice.
///
/// # Example
///
/// ```
/// use base::count_extremum;
/// assert_eq!(count_extremum(&[]), 0);
/// assert_eq!(count_extremum(&[0]), 1);
/// assert_eq!(count_extremum(&[0, 0]), 1);
/// assert_eq!(count_extremum(&[0, 1, 0]), 3);
/// assert_eq!(count_extremum(&[0, 1, 2, 3, 4, 4, 4, 3, 2, 1, 0]), 3);
/// assert_eq!(count_extremum(&[0, 1, 2, 2, 1, 1, 2, 3, 2]), 5);
/// ```
pub fn count_extremum(values: &[u32]) -> usize {
    unimplemented!()
}
