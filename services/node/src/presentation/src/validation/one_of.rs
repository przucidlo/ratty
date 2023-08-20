pub fn one_of(values: Vec<&str>, value: &str) -> garde::Result {
    if values.contains(&value) {
        Ok(())
    } else {
        Err(garde::Error::new(
            "value does not match any of expected values",
        ))
    }
}
