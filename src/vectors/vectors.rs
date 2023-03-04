// Vectors
pub fn vectors() -> i32 {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);

    x
}

#[cfg(test)]
mod tests {
    use super::vectors;

    #[test]
    fn test_vectors() {
        assert_eq!(vectors(), 60);
    }
}
