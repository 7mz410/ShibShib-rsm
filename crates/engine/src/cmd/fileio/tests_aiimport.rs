//! Illustrator EPS and `.ai` files open from the editing data they carry: layers, groups, names,
//! hidden objects, artboards, colours and images as they were; what the reader doesn't read yet
//! opens as before, with a warning saying why.

use std::io::Write as _;
use std::sync::Arc;

use serde_json::{Value, json};
use vectorcraft_doc::{Node, NodeKind};
use vectorcraft_testkit::ai;

use super::*;

fn open(s: &mut Session, name: &str, bytes: &[u8], params: Value) -> Result<Value> {
    let mut p = json!({"name": name, "dataBase64": vectorcraft_format::base64_encode(bytes)});
    if let (Some(p), Some(extra)) = (p.as_object_mut(), params.as_object()) {
        p.extend(extra.clone());
    }
    s.execute("document.open", &p)
}

fn zlib(data: &[u8]) -> Vec<u8> {
    let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    e.write_all(data).unwrap();
    e.finish().unwrap()
}

/// The `.ai` private data of `data`, compressed as such files compress it.
fn compressed(data: &[u8]) -> Vec<u8> {
    [&b"%AI12_CompressedData"[..], &zlib(data)].concat()
}

fn names(nodes: &[Arc<Node>]) -> Vec<String> {
    nodes.iter().map(|n| n.name.clone().unwrap_or_default()).collect()
}

/// The sample's structure, as [`ai::sample`] builds it.
fn check_sample(s: &Session) {
    let doc = &s.doc().unwrap().doc;
    assert_eq!(names(&doc.layers), ["Art", "Hidden", "Images"]);
    assert_eq!(doc.artboards.len(), 1);
    assert_eq!(doc.artboards[0].rect, vectorcraft_geom::Rect::new(0.0, 0.0, 200.0, 100.0));
    let art = doc.layers[0].children().unwrap();
    assert_eq!(names(art), ["Pair", "Ring", "Window", "Shaded", ""]);
    let pair = art[0].children().unwrap();
    assert_eq!(names(pair), ["Red", ""]);
    assert!(pair[0].visible && !pair[1].visible, "the hidden member stays hidden");
    assert!(matches!(art[1].kind, NodeKind::Compound { .. }));
    assert!(matches!(&art[2].kind, NodeKind::Group { clip: true, children } if matches!(children[0].kind, NodeKind::Path { clipping: true, .. })));
    assert!(matches!(art[3].appearance.fill().map(|f| &f.paint), Some(vectorcraft_color::Paint::Gradient(_))));
    assert_eq!((art[4].opacity, art[4].blend), (0.5, vectorcraft_color::BlendMode::Screen));
    let hidden = &doc.layers[1];
    assert!(!hidden.visible);
    let sub = &hidden.children().unwrap()[0];
    assert!(sub.locked && matches!(sub.kind, NodeKind::Layer { .. }));
    let image = &doc.layers[2].children().unwrap()[0];
    let NodeKind::Image(im) = &image.kind else { panic!("an image") };
    assert_eq!((im.width, im.height), (2, 1));
}

#[test]
fn an_illustrator_eps_opens_with_its_layers() {
    let mut s = Session::new();
    let r = open(&mut s, "art.eps", &ai::eps(&ai::sample_data()), json!({})).unwrap();
    assert_eq!((&r["format"], &r["warnings"]), (&json!("eps"), &json!([])), "{r}");
    check_sample(&s);
    // Saved as EPS and opened again, it comes back the same.
    let eps = s.execute("document.exportEps", &json!({})).unwrap();
    let bytes = vectorcraft_format::base64_decode(eps["dataBase64"].as_str().unwrap()).unwrap();
    let r = open(&mut s, "again.eps", &bytes, json!({})).unwrap();
    assert_eq!(r["restored"], json!(true), "{r}");
    check_sample(&s);
}

#[test]
fn an_illustrator_ai_file_opens_from_its_editing_data() {
    let mut s = Session::new();
    let r = open(&mut s, "art.ai", &ai::ai(&compressed(&ai::sample_data())), json!({})).unwrap();
    assert_eq!((&r["format"], &r["warnings"]), (&json!("ai"), &json!([])), "{r}");
    check_sample(&s);
    // Pages picked: the PDF's.
    let r = open(&mut s, "art.ai", &ai::ai(&compressed(&ai::sample_data())), json!({"pages": "1"})).unwrap();
    assert_eq!(names(&s.doc().unwrap().doc.layers).len(), 1, "{r}");
    // A PDF that happens to carry the data opens as a PDF.
    open(&mut s, "art.pdf", &ai::ai(&compressed(&ai::sample_data())), json!({})).unwrap();
    assert_ne!(names(&s.doc().unwrap().doc.layers), ["Art", "Hidden", "Images"]);
}

#[test]
fn what_the_reader_doesnt_read_opens_as_before_with_a_warning() {
    let text = ai::editing_data(200.0, 100.0, &ai::layer("Words", "/AI11Text :\n0 /FreeUndo ,\n0 /FrameIndex ,\n0 /StoryIndex ,\n;\n"));
    let mut s = Session::new();
    let r = open(&mut s, "words.eps", &ai::eps(text.as_bytes()), json!({})).unwrap();
    let first = r["warnings"][0].as_str().unwrap();
    assert!(first.contains("editing data couldn't be read") && first.contains("type") && first.contains("printed page"), "{r}");
    assert!(s.doc().unwrap().doc.layers.iter().any(|l| l.children().is_some_and(|c| !c.is_empty())), "the page's line");
    let r = open(&mut s, "words.ai", &ai::ai(&compressed(text.as_bytes())), json!({})).unwrap();
    let first = r["warnings"][0].as_str().unwrap();
    assert!(first.contains("type") && first.contains("PDF content"), "{r}");
    // Damaged data says so.
    let r = open(&mut s, "damaged.ai", &ai::ai(b"%AI12_CompressedData not zlib at all"), json!({})).unwrap();
    assert!(r["warnings"][0].as_str().unwrap().contains("zlib"), "{r}");
}
