pub(super) fn parse(text: &str) -> Result<Vec<Vec<String>>, String> {
    text.lines()
        .enumerate()
        .map(|(index, line)| {
            parse_line(line).map_err(|error| format!("line {}: {error}", index + 1))
        })
        .collect()
}

fn parse_line(line: &str) -> Result<Vec<String>, String> {
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut chars = line.chars().peekable();
    let mut quoted = false;
    while let Some(character) = chars.next() {
        match character {
            '"' if quoted && chars.peek() == Some(&'"') => {
                chars.next();
                cell.push('"');
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                cells.push(std::mem::take(&mut cell));
            }
            other => cell.push(other),
        }
    }
    if quoted {
        return Err("unterminated quoted cell".into());
    }
    cells.push(cell);
    Ok(cells)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parses_quoted_cells() {
        let rows = parse("a,\"b,c\",\"d\"\"e\"\n").unwrap();

        assert_eq!(rows[0], vec!["a", "b,c", "d\"e"]);
    }
}
