use super::{hrefs, srcs};

#[test]
fn extracts_double_single_and_uppercase_hrefs() {
    let links = hrefs(r#"<a href="a.html"></a><a HREF='b.html'></a>"#);

    assert_eq!(links, vec!["a.html", "b.html"]);
}

#[test]
fn ignores_unquoted_href_values() {
    let links = hrefs(r#"<a href=a.html></a>"#);

    assert!(links.is_empty());
}

#[test]
fn extracts_src_values() {
    let sources = srcs(r#"<script src="app.js"></script><img SRC='logo.png'>"#);

    assert_eq!(sources, vec!["app.js", "logo.png"]);
}
