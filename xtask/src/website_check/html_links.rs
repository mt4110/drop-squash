pub(super) fn hrefs(text: &str) -> Vec<String> {
    let mut hrefs = Vec::new();
    let mut rest = text;
    while let Some(index) = rest.to_ascii_lowercase().find("href") {
        rest = &rest[index + 4..];
        let Some((href, next)) = take_href(rest) else {
            continue;
        };
        hrefs.push(href.to_string());
        rest = next;
    }
    hrefs
}

fn take_href(text: &str) -> Option<(&str, &str)> {
    let text = text.trim_start();
    let text = text.strip_prefix('=')?.trim_start();
    let quote = text.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let text = &text[quote.len_utf8()..];
    let end = text.find(quote)?;
    Some((&text[..end], &text[end + quote.len_utf8()..]))
}

#[cfg(test)]
mod tests;
