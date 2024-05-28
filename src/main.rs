// src/main.rs

fn main() {
    let (a, b) = (5, 10);
    let (a, b) = swap(a, b);
    println!("Swapped values: a = {}, b = {}", a, b);
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

#[cfg(test)]
mod tests {
    use super::swap;

    #[test]
    fn test_swap() {
        let (a, b) = (1, 2);
        let (a, b) = swap(a, b);
        assert_eq!((a, b), (2, 1));
    }

    #[test]
    fn test_swap_same_values() {
        let (a, b) = (5, 5);
        let (a, b) = swap(a, b);
        assert_eq!((a, b), (5, 5));
    }

    #[test]
    fn test_swap_negative_values() {
        let (a, b) = (-3, -7);
        let (a, b) = swap(a, b);
        assert_eq!((a, b), (-7, -3));
    }
}
