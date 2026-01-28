pub fn indent_lines(indentation: &str, input: impl AsRef<str>) -> String {
    input
        .as_ref()
        .lines()
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                format!("{}{}", indentation, line)
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
