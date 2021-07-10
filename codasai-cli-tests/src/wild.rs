use std::fmt::{Debug, Display};

/// A string that can be compared against a wild pattern.
///
/// A wild pattern is a string that might contain `[..]` in its body acting as a wildcard.
pub struct WildStr<'a>(&'a str);

impl<'a> From<&'a str> for WildStr<'a> {
    fn from(inner: &'a str) -> Self {
        WildStr(inner)
    }
}

impl<'a> PartialEq<&str> for WildStr<'a> {
    fn eq(&self, wild_pattern: &&str) -> bool {
        let mut rest = self.0;

        // Courtesy of `cargo`'s source code
        for (i, part) in wild_pattern.split("[..]").enumerate() {
            match rest.find(part) {
                Some(j) => {
                    if i == 0 && j != 0 {
                        return false;
                    }
                    rest = &rest[j + part.len()..];
                }
                None => return false,
            }
        }
        rest.is_empty() || wild_pattern.ends_with("[..]")
    }
}

impl<'a> Debug for WildStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.0, f)
    }
}

impl<'a> Display for WildStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.0, f)
    }
}
