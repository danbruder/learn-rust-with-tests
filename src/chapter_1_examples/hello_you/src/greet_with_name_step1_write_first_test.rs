fn greet() -> String {
    String::from("Hello, World!")
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet_with_name() {
        let got = greet(String::from("Chris"));
        let want = "Hello, Chris";

        assert_eq!(got, want);
    }
}
