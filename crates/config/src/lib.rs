#[derive(Debug)]
pub struct Config<'a> {
    regex: &'a str,
    file_name: &'a str,
    // TODO: I see a design flaw here that I can't specificy a lifetime specifier with file_content as it's owned value
    // but technically (opiniated) the Config's lifetime should be tied to its source (like command line args)
    file_content: String,
    case_insensitivity: bool,
}

impl<'a> Config<'a> {
    pub fn get_regex(&self) -> &'a str {
        self.regex
    }

    pub fn get_file_content(&self) -> &String {
        &self.file_content
    }

    pub fn get_file_name(&self) -> &'a str {
        self.file_name
    }

    pub fn get_case_insensitivity(&self) -> bool {
        self.case_insensitivity
    }

    // we send a ref to slice of String, it helps us to NOT specify the lifetime explicitly
    pub fn parse(arguments: &'a [String], is_case_insensitive: bool) -> Result<Self, String> {
        let regex_input = arguments
            .get(0)
            .ok_or("Regex input expected as first argument")?;
        let file_input = arguments
            .get(1)
            .ok_or("File input expected as second argument")?;

        let file_content = std::fs::read_to_string(file_input)
            // map_err to transform the error E -> F
            .map_err(|e| format!("Error opening the file \"{}\",\n Error: {e}", file_input))?;

        Ok(Config {
            regex: regex_input,
            file_content,
            file_name: file_input,
            case_insensitivity: is_case_insensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
