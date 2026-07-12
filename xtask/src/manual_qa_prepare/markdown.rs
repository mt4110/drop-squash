pub(super) type Field = (&'static str, String);

pub(super) fn print_fields(fields: &[Field]) {
    println!("manual QA Markdown fields:");
    for (label, value) in fields {
        println!("| {label} | {value} |");
    }
}

#[cfg(test)]
mod tests {
    use super::Field;

    #[test]
    fn field_type_accepts_manual_qa_rows() {
        let fields: Vec<Field> = vec![("App build", "DropSquash 0.1.0 git abc1234".into())];

        assert_eq!(fields[0].0, "App build");
    }
}
