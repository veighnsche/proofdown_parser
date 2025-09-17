use proofdown_ast::{Attr, Block, Component, Document};
use proofdown_validate::{validate, Limits, ValidateError};

fn doc_with(comp: Component) -> Document {
    Document {
        blocks: vec![Block::Component(comp)],
    }
}

fn comp(name: &str, attrs: &[(&str, &str)]) -> Component {
    Component {
        name: name.to_string(),
        attrs: attrs
            .iter()
            .map(|(k, v)| Attr {
                key: (*k).into(),
                value: (*v).into(),
            })
            .collect(),
        children: vec![],
        self_closing: true,
    }
}

#[test]
fn json_pointer_valid_and_invalid() {
    let ok = comp(
        "artifact.json",
        &[
            ("id", "x"),
            ("json_pointer", "/a/b"),
            ("collapsed", "true"),
            ("depth", "2"),
        ],
    );
    validate(&doc_with(ok), Some(&Limits::default())).expect("valid json_pointer");

    let bad = comp("artifact.json", &[("id", "x"), ("json_pointer", "a/b")]);
    let err = validate(&doc_with(bad), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::AttrType { .. }));
}

#[test]
fn image_caption_and_bounds() {
    let ok = comp(
        "artifact.image",
        &[
            ("id", "img"),
            ("alt", "desc"),
            ("caption", "An image"),
            ("max_height", "512"),
        ],
    );
    validate(&doc_with(ok), Some(&Limits::default())).expect("image with caption ok");

    let bad = comp(
        "artifact.image",
        &[("id", "img"), ("alt", "desc"), ("max_height", "42")],
    );
    let err = validate(&doc_with(bad), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::AttrBound { .. }));
}

#[test]
fn table_columns_validation() {
    let ok = comp(
        "artifact.table",
        &[
            ("id", "t"),
            ("columns", "/a/b,foo,bar"),
            ("caption", "Hello"),
            ("kind", "api"),
        ],
    );
    validate(&doc_with(ok), Some(&Limits::default())).expect("table columns ok");

    let bad = comp("artifact.table", &[("id", "t"), ("columns", " /bad~2 ")]);
    let err = validate(&doc_with(bad), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::AttrType { .. }));
}

#[test]
fn text_max_lines_bounds() {
    let ok = comp(
        "artifact.text",
        &[("id", "t"), ("max_lines", "100"), ("caption", "Snippet")],
    );
    validate(&doc_with(ok), Some(&Limits::default())).expect("text ok");

    let bad = comp("artifact.text", &[("id", "t"), ("max_lines", "0")]);
    let err = validate(&doc_with(bad), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::AttrBound { .. }));
}
