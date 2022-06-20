use wasm_bindgen::prelude::*;
use linkify::{LinkFinder, LinkKind};

#[wasm_bindgen]
pub fn linkify_text(text: &str, allow_without_scheme: bool) -> String {
    let mut link_finder = LinkFinder::new();
    link_finder.url_must_have_scheme(!allow_without_scheme);
    let mut bytes = Vec::new();
    for span in link_finder.spans(text) {
        match span.kind() {
            Some(LinkKind::Url) => {
                let mut url = span.as_str().to_string();
                if !url.contains(":") {
                    url.insert_str(0, "https://");
                }
                bytes.extend_from_slice(b"<a href=\"");
                escape(&url, &mut bytes);
                bytes.extend_from_slice(b"\" title=\"URL\">");
                escape(span.as_str(), &mut bytes);
                bytes.extend_from_slice(b"</a>");
            }
            Some(LinkKind::Email) => {
                bytes.extend_from_slice(b"<a href=\"mailto:");
                escape(span.as_str(), &mut bytes);
                bytes.extend_from_slice(b"\" title=\"email\">");
                escape(span.as_str(), &mut bytes);
                bytes.extend_from_slice(b"</a>");
            }
            _ => {
                escape(span.as_str(), &mut bytes);
            }
        }
    }
    String::from_utf8(bytes).expect("added bytes are all ASCII")
}

fn escape(text: &str, dest: &mut Vec<u8>) {
    for c in text.bytes() {
        match c {
            b'&' => dest.extend_from_slice(b"&amp;"),
            b'<' => dest.extend_from_slice(b"&lt;"),
            b'>' => dest.extend_from_slice(b"&gt;"),
            b'"' => dest.extend_from_slice(b"&quot;"),
            b'\'' => dest.extend_from_slice(b"&#39;"),
            _ => dest.push(c),
        }
    }
}
