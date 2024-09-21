use minigrep_config::Config;

pub type SearchResults<'a> = Vec<(usize, &'a str)>;

pub fn grep<'a>(config: &'a Config<'a>) -> SearchResults<'a> {
    search(
        config.get_regex(),
        config.get_file_content(),
        config.get_case_insensitivity(),
    )
}

pub fn search<'a>(
    search_input: &str,
    file_content: &'a str,
    is_case_insensitive: bool,
) -> SearchResults<'a> {
    file_content
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            if !is_case_insensitive {
                line.contains(search_input)
            } else {
                line.to_lowercase().contains(&search_input.to_lowercase())
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_search_result() {
        assert_eq!(
            search(
                "hell",
                "hey!hello world\nhow are you doing!\nIt's really hell here\ngo to Hell!", false
            ),
            vec![(0, "hey!hello world"), (2, "It's really hell here")],
            "Two results are expected out of three lines, since \"hell\" matches with \"hello\" and \"hell\""
        );
    }

    #[test]
    fn search_case_ininsensitive() {
        assert_eq!(
            search(
                "hell",
                "hey!hello world\nhow are you doing!\nIt's really hell here\ngo to Hell!", true // NOTE: True here
            ),
            vec![(0, "hey!hello world"), (2, "It's really hell here"), (3, "go to Hell!")],
            "Three results are expected out of three lines, since \"hell\" matches ininsensitively with \"hello\", \"hell\" and \"Hell\"");
    }
}
