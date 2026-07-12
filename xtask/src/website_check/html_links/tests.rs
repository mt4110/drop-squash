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
fn extracts_form_action_values() {
    let actions = actions(r#"<form action="submit.html"></form><form ACTION='buy.html'>"#);

    assert_eq!(actions, vec!["submit.html", "buy.html"]);
}
