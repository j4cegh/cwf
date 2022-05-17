use std::fs::File;
use std::io::Read;
use kuchiki::{ElementData, NodeDataRef};
use kuchiki::traits::TendrilSink;

fn do_replace(elem: NodeDataRef<ElementData>) {
    let mut attrs = elem.attributes.borrow_mut();
    let name = elem.name.local.to_string();

    if name == "script" {
        if let Some(src) = attrs.get_mut("src") {
            if let Some(x) = src.strip_suffix(".ts") {
                *src = format!("{}.js", x);
            }
        }

        attrs.insert("type", "module".to_string());
    }
}

pub fn replace_ts(html: &str) -> String {
    let doc = kuchiki::parse_html().one(html);
    let head = doc.select_first("head").unwrap();
    let body = doc.select_first("body").unwrap();

    for elem in head.as_node().children().filter_map(|n| n.into_element_ref()) {
        do_replace(elem);
    }

    for elem in body.as_node().children().filter_map(|n| n.into_element_ref()) {
        do_replace(elem);
    }

    doc.to_string()
}

pub fn load_page(loc: &str) -> String {
    println!("Location: src/{}", loc);
    let mut file = File::open(format!("src/{}", loc)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
