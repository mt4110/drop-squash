pub(super) fn hrefs(text: &str) -> Vec<String> {
    attr_values(text, "href")
}

pub(super) fn srcs(text: &str) -> Vec<String> {
    let mut values = attr_values(text, "src");
    for srcset in attr_values(text, "srcset") {
        values.extend(srcset_candidates(&srcset));
    }
    values
}

pub(super) fn actions(text: &str) -> Vec<String> {
    let mut values = attr_values(text, "action");
    values.extend(attr_values(text, "formaction"));
    values
}

fn attr_values(text: &str, name: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut rest = text;
    while let Some(index) = rest.to_ascii_lowercase().find(name) {
        let before = rest[..index].chars().next_back();
        rest = &rest[index + name.len()..];
        if before.is_some_and(is_attr_name_char) {
            continue;
        }
        let Some((href, next)) = take_href(rest) else {
            continue;
        };
        values.push(href.to_string());
        rest = next;
    }
    values
}

fn is_attr_name_char(value: char) -> bool {
    value.is_ascii_alphanumeric() || value == '-' || value == '_'
}

fn srcset_candidates(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter_map(|candidate| candidate.split_whitespace().next())
        .filter(|candidate| !candidate.is_empty())
        .map(str::to_string)
        .collect()
}

fn take_href(text: &str) -> Option<(&str, &str)> {
    let text = text.trim_start();
    let text = text.strip_prefix('=')?.trim_start();
    let quote = text.chars().next()?;
    if quote == '"' || quote == '\'' {
        let text = &text[quote.len_utf8()..];
        let end = text.find(quote)?;
        return Some((&text[..end], &text[end + quote.len_utf8()..]));
    }
    let end = text
        .find(|character: char| character.is_whitespace() || character == '>')
        .unwrap_or(text.len());
    (end > 0).then_some((&text[..end], &text[end..]))
}

#[cfg(test)]
mod tests;
