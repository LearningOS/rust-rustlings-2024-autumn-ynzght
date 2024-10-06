// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let s1 = String::from(input);
    let s2 = s1.trim_start();
    let s3 = s2.trim_end();
    s3.to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut s2 = String::from(input);
    s2.push_str(" world!");
    s2
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let new_s = input.replace("cars", "balloons");
    new_s
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
