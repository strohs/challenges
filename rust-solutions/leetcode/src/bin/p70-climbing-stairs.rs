/// # Problem 70 - Climbing Stairs
/// You are climbing a staircase. It takes n steps to reach the top.
///
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

pub fn climb_stairs(n: i32) -> i32 {
    match n {
        _ if n < 2 => n,
        _ => {
            let mut first = 1;
            let mut second = 2;
            let mut temp = 0;

            for _ in 3..=(n as usize) {
                temp = second;
                second += first;
                first = temp;
            }
            second
        }
    }
}



fn main() {}

#[cfg(test)]
mod tests {
    use crate::climb_stairs;

    #[test]
    fn example1() {
        assert_eq!(climb_stairs(2), 2)
    }

    #[test]
    fn example2() {
        assert_eq!(climb_stairs(3), 3)
    }

    #[test]
    fn example3() {
        assert_eq!(climb_stairs(4), 5)
    }

    #[test]
    fn example45() {
        assert_eq!(climb_stairs(45), 1836311903);
    }
}