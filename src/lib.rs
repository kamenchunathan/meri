pub fn init() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::init;

    #[test]
    fn always() {
        assert!(init());
    }
}
