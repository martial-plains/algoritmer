/// Remove duplicates from sentence
pub fn remove_duplicates(text: String) -> String {
    let modified_text_vec = text.split_whitespace().collect::<Vec<&str>>();
    let modified_text_vec_deduped = {
        let mut values: Vec<&str> = vec![];
        for t in modified_text_vec {
            if !values.contains(&t) {
                values.push(t);
            }
        }
        values
    };

    modified_text_vec_deduped.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Python is great and Java is also great".to_string(), "Python is great and Java also".to_string())]
    fn check_removed_duplicates(text: String, expected: String) {
        let actual = remove_duplicates(text);
        assert_eq!(expected, actual)
    }
}
