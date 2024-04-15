pub fn indent_lines(indentation: &str, input: impl AsRef<str>) -> String {
    input
        .as_ref()
        .lines()
        .map(|line| format!("{}{}", indentation, line))
        .collect::<Vec<String>>()
        .join("\n")
}
