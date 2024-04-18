// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    let mut s = String::new();
    let str2rep = vec!["", " "];
    let mut ilist: Vec<&str> = input.split("").collect();
    let mut frontReplace: bool = true;
    let mut backReplace: bool = true;
    println!("{:?}", ilist);
    while frontReplace || backReplace {
        frontReplace = str2rep.contains(&ilist[0]);
        backReplace = str2rep.contains(&ilist[ilist.len() - 1]);
        if frontReplace {
            ilist.remove(0);
        }
        if backReplace {
            ilist.pop();
        }
    }
    ilist.join("")
}

fn compose_me(input: &str) -> String {
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    let mut ilist: Vec<&str> = input.split(" ").collect();
    for word in ilist.iter_mut() {
        if *word == "cars" {
            *word = "balloons"
        }
    }
    ilist.join(" ")
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
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
