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
    let mut p: u32 = 0;
    assert_eq!(s1.len(), s2.len());
    // for i in 0..s1.len() {
    //     p += s1[i] * s2[i];
    // }
    let iter = s1.iter().zip(s2.iter());
    for (e1, e2) in iter {
        p += e1 * e2;
    }
    p
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
    let mut iter = slice.iter();
    if let Some(mut f) = iter.next() {
        for e in iter {
            if e < f {
                return false;
            }
            f = e;
        }
    }
    true
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
    assert_eq!(s.len() % 2, 0);
    let mut v = Vec::<(u32, u32)>::new();
    let mut iter = s.iter();
    for _ in 0..s.len()/2 {
        v.push((*iter.next().unwrap(), *iter.next().unwrap()));
    }
    v
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
    let even_slice = slice.iter().filter(|x| *x % 2 == 0);
    let mut sum = 0;
    for e in even_slice.take(n) {
        sum += e;
    }
    sum
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
    let mut res = (0, 0);
    for e in slice {
        if *e == 0 {
            res.0 += 1;
        }
        else if *e == 1 {
            res.1 += 1;
        }
    }
    res
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
    for (i, e) in slice.iter().enumerate() {
        if *e == target {
            return Some(i)
        }
    }
    None
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
    let mut res = String::new();
    for &s in strings {
        for c in s.chars() {
            if c.to_digit(10) != None {
                res.push(c);
            }
        }
    }
    res
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
    let mut i = integer;
    let mut sum = 0;
    while i / 10 > 0 {
        sum += i % 10;
        i = i / 10;
    }
    sum + i
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
    let mut i = 0;
    let mut sum = 0.;
    while sum < limit {
        i += 1;
        sum += f(i);
    }
    i
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
    let mut res = vec![start];
    for e in intermediate.into_iter() {
        res.push(e);
    }
    res.push(end);
    res
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
    let mut iter = values.into_iter();
    let mut count_max = 0;
    let mut count_min = 0;
    if let Some(mut last) = iter.next() {
        let mut state = None;
        count_min += 1;
        
        for e in iter {
            if state == None && e != last {
                count_min += 1;
            }
            else if state == Some(1) && e <= last {
                count_max += 1;
            }
            else if state == Some(-1) && e >= last {
                count_min += 1;
            }
    
            if e > last {
                state = Some(1);
            }
            else if e == last {
                state = Some(0);
            }
            else {
                state = Some(-1);
            }
            last = e;
            
        }
        count_min + count_max
    }
    else {
        0
    }

}


