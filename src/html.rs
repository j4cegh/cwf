use kuchiki;
use kuchiki::traits::TendrilSink;

pub fn replace_ts(html: &str) {
    let mut doc = kuchiki::parse_html().one(html);
    let mut nodes = doc.children();

    for node in nodes {}
}
