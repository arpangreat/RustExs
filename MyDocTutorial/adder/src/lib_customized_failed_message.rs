pub fn greetings(name: &str) -> String {
    String::from("Hello!!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greetings_contains_name() {
        let result = greetings("Swastik");
        assert!(
            result.contains("Swastik"),
            "Greetings did not contain name, value was `{}`",
            result
        );
    }
}
