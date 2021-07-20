use super::{verify};

#[cfg(tests)]
mod tests {
    #[test]
    fn test_verify () -> bool {
        assert_eq!(verify("/home"), true)
    }
}
