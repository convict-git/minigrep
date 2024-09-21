use minigrep_config::Config;
use minigrep_core::grep;

fn main() {
    let command_line_args = std::env::args()
        .into_iter()
        .skip(1)
        // Here you need mention the type B for collect to specify the collection
        .collect::<Vec<_>>();
    println!("Command line arguments to minigrep {:?}", command_line_args);

    let is_case_insensitive = std::env::var("IGNORE_CASE").is_ok();

    let config =
        Config::parse(&command_line_args, is_case_insensitive).unwrap_or_else(|error_msg| {
            eprintln!("{error_msg}");
            std::process::exit(1); // NOTE: instead of panic for soft-end (no panic logs) and with an exit code
        });

    println!(
        "Searching for the query {}, in file {}",
        config.get_regex(),
        config.get_file_name()
    );

    println!("{:?}", grep(&config));
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {}
}
