#[cfg(test)]
mod tests {
    use homework0::your_code;

    #[test]
    fn sample_test() {
        let res = your_code();
        assert!((res.x - 1.7071).abs() < 0.0001);
        assert!((res.y - 4.1213).abs() < 0.0001);
    }

    #[test]
    fn your_test() {}
}
