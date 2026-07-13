pub(super) type Field = (&'static str, String);

pub(super) fn print_fields(fields: &[Field]) {
    println!("manual QA Markdown fields:");
    for row in rows(fields) {
        println!("{row}");
    }
}

pub(super) fn rows(fields: &[Field]) -> Vec<String> {
    fields
        .iter()
        .map(|(label, value)| format!("| {label} | {value} |"))
        .collect()
}

#[cfg(test)]
mod tests;
