// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a


fn trim_me(input: &str) -> String {
    let trimmed = input.trim(); // TODO: Trim the string!
    trimmed.to_string() // TODO: Convert the trimmed string to a String type!
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut composed = String::new();
    composed.push_str(input); // TODO: Add the input string to the new string!
    composed.push_str(" world!"); // TODO: Add " world!" to the new string!
    composed // TODO: Return the new string!
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
   let replaced = input.replace("cars", "balloons"); // TODO: Replace the string!
    replaced
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
