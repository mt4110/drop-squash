use super::{actions, hrefs, srcs};

#[test]
fn extracts_double_single_and_uppercase_hrefs() {
    let links = hrefs(r#"<a href="a.html"></a><a HREF='b.html'></a>"#);

    assert_eq!(links, vec!["a.html", "b.html"]);
}

#[test]
fn extracts_unquoted_href_values() {
    let links = hrefs(r#"<a href=a.html></a>"#);

    assert_eq!(links, vec!["a.html"]);
}

#[test]
fn ignores_attribute_names_that_only_end_with_href() {
    let links = hrefs(r#"<div data-href="checkout.html"></div><a href="pricing.html"></a>"#);

    assert_eq!(links, vec!["pricing.html"]);
}

#[test]
fn extracts_src_values() {
    let sources = srcs(r#"<script src="app.js"></script><img SRC='logo.png'>"#);

    assert_eq!(sources, vec!["app.js", "logo.png"]);
}

#[test]
fn extracts_srcset_candidates() {
    let sources = srcs(r#"<img srcset="small.png 1x, large.png 2x">"#);

    assert_eq!(sources, vec!["small.png", "large.png"]);
}

#[test]
fn extracts_uppercase_srcset_candidates() {
    let sources = srcs(r#"<img SRCSET='small.png 400w, large.png 800w'>"#);

    assert_eq!(sources, vec!["small.png", "large.png"]);
}

#[test]
fn extracts_unquoted_src_values() {
    let sources = srcs(r#"<script src=app.js></script>"#);

    assert_eq!(sources, vec!["app.js"]);
}

#[test]
fn extracts_unquoted_self_closing_src_values() {
    let sources = srcs(r#"<img src=logo.png />"#);

    assert_eq!(sources, vec!["logo.png"]);
}

#[test]
fn extracts_form_action_values() {
    let actions = actions(r#"<form action="submit.html"></form><form ACTION='buy.html'>"#);

    assert_eq!(actions, vec!["submit.html", "buy.html"]);
}

#[test]
fn extracts_unquoted_form_action_values() {
    let actions = actions(r#"<form action=buy.html></form>"#);

    assert_eq!(actions, vec!["buy.html"]);
}
