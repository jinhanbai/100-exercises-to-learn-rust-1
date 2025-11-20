// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.
use std::thread::scope;
pub fn sum(v: Vec<i32>) -> i32 {
    scope(|scope| {
        let (left, right) = v.split_at(v.len() / 2);
        let left_handle = scope.spawn(move || {
            let sum = left.iter().sum::<i32>();
            println!("Left sum: {}", sum);
            sum
        });
        let right_handle = scope.spawn(move || {
            let sum = right.iter().sum::<i32>();
            println!("Right sum: {}", sum);
            sum
        });
        let result = left_handle.join().unwrap() + right_handle.join().unwrap();
        println!("Final result: {}", result);
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
