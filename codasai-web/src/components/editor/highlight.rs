/// # Returns
///
/// - The index of the **element** in `substrings` that contains the last character in `string`
/// - The index of the character in **element** that corresponds to the last character in `string`
fn find_split_point(string: &str, substrings: &[&str]) -> (usize, usize) {
    let mut len_so_far = 0;

    for (i, substring) in substrings.into_iter().enumerate() {
        let remaining_len = string.len() - len_so_far;

        if substring.len() <= remaining_len {
            len_so_far += substring.len();
        } else {
            return (i, string.len() - len_so_far - 1);
        }
    }

    // If we got here, it means that all substrings are part of `string`
    (substrings.len(), substrings.last().map(|s| s.len() - 1).unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_split_point() {
        let string = "Hey what is going on there?";
        let substrings = &["Hey wh", "at ", "is goin", "g on there?, eve", "ryone"];
        assert_eq!(find_split_point(&string, substrings), (3, 11));
    }
}

