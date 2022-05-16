use kuchiki;
use kuchiki::traits::TendrilSink;

pub fn replace_ts(html: &str) -> String {
    let doc = kuchiki::parse_html().one(html);
    let head = doc.select_first("head").unwrap();

    for elem in head.as_node().children().filter_map(|n| n.into_element_ref()) {
        let mut attrs = elem.attributes.borrow_mut();
        if let Some(src) = attrs.get_mut("src") {
            if let Some(x) = src.strip_suffix(".ts") {
                *src = format!("{x}.js");
            }
        }

        // import it as a module
        attrs.insert("type", "module".to_string());
    }

    doc.to_string()
}
