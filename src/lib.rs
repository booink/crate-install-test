#[cfg(test)]
mod tests {
    #[test]
    fn test_crate1_ver1() {
        assert_eq!(
            crate_test_1::crate1_ver1::Crate1Ver1::hello(),
            "Hello version 1.1.0 in crate 1.".to_string()
        );
    }

    #[test]
    fn test_crate2_ver1() {
        assert_eq!(
            crate_test_2::crate2_ver1::Crate2Ver1::hello(),
            "Hello version 1.0 in crate 2.".to_string()
        );
    }
}
