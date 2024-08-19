fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string.
    let mut str = String::new();

    let mut strt : usize = 0;
    let mut endt : usize = 0;

    for ch in input.chars().enumerate() {
        if ch.1 != ' ' {
            strt = ch.0;
            break;
        }
    }

    for ch in input.chars().rev().enumerate() {
        if ch.1 != ' '{
            endt = ch.0;
            break;
        }
    }

    input[strt..input.len()-endt].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    let s = input.to_string();
    s+" world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    input.replace("cars","balloons")
}

fn main() {
    // You can optionally experiment here.
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
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
