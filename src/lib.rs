/// Return the dot product of s1 by s2.

pub fn dot_product(s1: &[u32], s2: &[u32]) -> u32 {
    s1.iter().zip(s2.iter()).map(|(a, b)| a * b).sum()
}

/// Return if given slice is sorted.

pub fn is_sorted(slice: &[u32]) -> bool {
    slice.windows(2).all(|a| a[0] <= a[1])
}

/// Transform a slice n u32 (n even) into a vector of n/2 couples of u32.

pub fn into_couples(s: &[u32]) -> Vec<(u32, u32)> {
    let mut vec: Vec<(u32, u32)> = vec![];
    for elem in s.windows(2).step_by(2) {
        vec.push((elem[0], elem[1]));
    }
    return vec;
}

/// Return the sum of the n first even integers of the given slice.
/// If not enough sum all of them.

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

pub fn number_of_a(s: &str) -> usize {
    s.chars()
        .fold(0, |nb, elem| if elem == 'a' { nb + 1 } else { nb })
}

/// Return the number of 0 and the number of 1 in given slice.

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

pub fn first_place(slice: &[u32], target: u32) -> Option<usize> {
    slice.iter().position(|&a| a == target)
}

/// Return the String formed by all digits in given strings.

pub fn digits(strings: &[&str]) -> String {
    let numbers = "0123456789";
    strings.iter().fold(String::from(""), |res, str| {
        res + &(str.chars().filter(|c| numbers.contains(*c))).collect::<String>()
    })
}

/// Return the sum of digits of given integer.

pub fn digits_sum(integer: u32) -> u32 {
    integer
        .to_string()
        .as_str()
        .chars()
        .fold(0, |sum, char| sum + char.to_digit(10).unwrap())
}

/// Return for which i the sum of f(i) (from 1 to infinity) is greater than n.

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
    unimplemented!();
}
