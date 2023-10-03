/// Return the dot product of s1 by s2.
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
    s1.iter().zip(s2.iter()).map(|(a, b)| a * b).sum()
}

/// # Example:
///
/// ```
/// use base::is_sorted;
/// assert!(is_sorted(&[1,2,3,4]));
/// assert!(!is_sorted(&[1,3,2,4]));
/// assert!(!is_sorted(&[1,2,4,3]));
/// ```
pub fn is_sorted(slice: &[u32]) -> bool {
    slice.windows(2).all(|a| a[0] <= a[1])
}

/// Transform a slice n u32 (n even) into a vector of n/2 couples of u32.
/// Return if given slice is sorted.
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
    let mut vec: Vec<(u32, u32)> = vec![];
    for elem in s.windows(2).step_by(2) {
        vec.push((elem[0], elem[1]));
    }
    return vec;
}

/// Return the sum of the n first even integers of the given slice.
/// If not enough sum all of them.
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
    let mut i = 0;
    slice.iter().fold(0, |sum, elem| {
        if i < n && (elem % 2 == 0) {
            i = i + 1;
            sum + elem
        } else {
            sum
        }
    })
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
    s.chars()
        .fold(0, |nb, elem| if elem == 'a' { nb + 1 } else { nb })
}

/// Return the number of 0 and the number of 1 in given slice.
/// # Example
///
/// ```
/// use base::number_of_01;
/// assert_eq!(number_of_01(&[0, 1, 1, 1, 4, 0, 1, 0, 2]), (3, 4));
/// assert_eq!(number_of_01(&[2, 5, 1, 1, 2]), (0, 2));
/// ```
pub fn number_of_01(slice: &[u32]) -> (usize, usize) {
    slice.iter().fold((0, 0), |(nb0, nb1), elem| {
        if *elem == 0 {
            (nb0 + 1, nb1)
        } else if *elem == 1 {
            (nb0, nb1 + 1)
        } else {
            (nb0, nb1)
        }
    })
}

/// Return the position of the first occurence of given number in given slice.

/// # Example:
/// ```
/// use base::first_place;
/// let t = [3, 1, 5, 2, 7 ,6];
/// assert_eq!(first_place(&t, 5), Some(2));
/// assert_eq!(first_place(&t, 3), Some(0));
/// assert_eq!(first_place(&t, 8), None);
/// ```
pub fn first_place(slice: &[u32], target: u32) -> Option<usize> {
    slice.iter().position(|&a| a == target)
}

/// Return the String formed by all digits in given strings.

/// # Example:
///
/// ```
/// use base::digits;
/// assert_eq!(digits(&["5 * 5", "=25"]), String::from("5525"));
/// assert_eq!(digits(&["1+2", "*3", "=9"]), String::from("1239"));
/// ```
pub fn digits(strings: &[&str]) -> String {
    let numbers = "0123456789";
    strings.iter().fold(String::from(""), |res, str| {
        res + &(str.chars().filter(|c| numbers.contains(*c))).collect::<String>()
    })
}

/// Return the sum of digits of given integer.
///
/// # Example:
/// ```
/// use base::enough_sum;
/// assert_eq!(enough_sum(|i| 1.0, 9.5), 10);
/// assert_eq!(enough_sum(|i| 1.0/(i as f64), 2.0), 4); // 1+1/2+1/3+1/4
/// ```
pub fn digits_sum(integer: u32) -> u32 {
    integer
        .to_string()
        .as_str()
        .chars()
        .fold(0, |sum, char| sum + char.to_digit(10).unwrap())
}

/// Return for which i the sum of f(i) (from 1 to infinity) is greater than n.
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
    let mut i = 1;
    let mut res: f64 = f(i);
    while res < limit {
        i = i + 1;
        res = res + f(i);
    }
    return i;
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
    let mut vec: Vec<u32> = vec![];
    vec.push(start);
    intermediate.into_iter().for_each(|a| vec.push(a));
    vec.push(end);
    return vec;
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
    if values.len() == 0 {
        return 0;
    };
    if values.len() == 1 {
        return 1;
    };
    if values.len() == 2 {
        if values[0] != values[1] {
            return 2;
        } else {
            return 1;
        }
    };
    let mut vec: Vec<u32> = vec![];
    values.windows(2).for_each(|a| {
        if a[0] != a[1] {
            vec.push(a[0])
        }
    });
    if (values.len()%2 == 1) && (values[values.len()-1] != values[values.len()-2]) {vec.push(values[values.len()-1])};
    vec.windows(3).fold(0, |acc, a| {
        if ((a[1] > a[0]) && (a[1] > a[2])) || ((a[1] < a[0]) && (a[1] < a[2])) {
            acc + 1
        } else {
            acc
        }
    })+2
}
