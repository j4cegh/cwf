use std::fs::File;
use std::io::Read;
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

        let name = elem.name.local.to_string();

        if name == "script" {
            attrs.insert("type", "module".to_string());
        }
    }

    doc.to_string()
}

pub fn load_page(loc: &str) -> String {
    println!("Loc: src/{}", loc);
    let mut file = File::open(format!("src/{}", loc)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
