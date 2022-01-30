#[cfg(test)]
mod tests {
    #[test]
    fn test_private_crate1_ver1() {
        assert_eq!(
            private_crate_test_1::private_crate1_ver1::PrivateCrate1Ver1::hello(),
            "Hello version 1.0 in private crate 1.".to_string()
        );
    }

    #[test]
    fn test_crate2_ver1() {
        assert_eq!(
            private_crate_test_2::private_crate2_ver1::PrivateCrate2Ver1::hello(),
            "Hello version 1.0 in private crate 2.".to_string()
        );
    }
}
