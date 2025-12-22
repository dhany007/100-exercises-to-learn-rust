// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    // Split the slice into two halves
    let mid = slice.len() / 2;
    let (left, right) = slice.split_at(mid);
    
    // Spawn a thread to sum the left half
    let left_handle = thread::spawn(move || {
        left.iter().sum::<i32>()
    });
    
    // Spawn a thread to sum the right half
    let right_handle = thread::spawn(move || {
        right.iter().sum::<i32>()
    });
    
    // Wait for both threads to complete and combine results
    let left_sum = left_handle.join().unwrap();
    let right_sum = right_handle.join().unwrap();
    
    left_sum + right_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
