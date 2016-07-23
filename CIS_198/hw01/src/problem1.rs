/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    slice.iter()
         .fold(0, |acc, x: &i32| acc + x)
}

/// Deduplicates items in the vector `vs`. Produces a vector
/// containing the first instance of each distinct element of `vs`,
/// preserving the origin order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut nv: Vec<i32> = vec![];

    for i in vs {
        match nv.contains(i) {
            true  => nv.push(*i),
            false => continue,
        }
    }
    nv
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut nv: Vec<i32> = vec![];

    for i in vs {
        match pred(*i) {
            true  => nv.push(*i),
            false => continue,
        }
    }
    nv
}
