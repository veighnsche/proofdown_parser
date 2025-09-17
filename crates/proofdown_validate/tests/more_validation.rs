use proofdown_ast::{Attr, Block, Component, Document};
use proofdown_validate::{validate, Limits, ValidateError};

fn doc_with(comp: Component) -> Document { Document { blocks: vec![Block::Component(comp)] } }

fn comp(name: &str, attrs: &[(&str, &str)]) -> Component {
    Component {
        name: name.to_string(),
        attrs: attrs.iter().map(|(k,v)| Attr { key: (*k).into(), value: (*v).into() }).collect(),
        children: vec![],
        self_closing: true,
    }
}

#[test]
fn unknown_component_errors() {
    let c = comp("foo", &[("id", "x")]);
    let err = validate(&doc_with(c), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::UnknownComponent { .. }));
}

#[test]
fn grid_unknown_attribute_errors() {
    let c = comp("grid", &[("cols", "2"), ("gap", "8"), ("foo", "bar")]);
    let err = validate(&doc_with(c), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::UnknownAttribute { .. }));
}

#[test]
fn artifact_missing_id_errors() {
    let c = comp("artifact.json", &[("depth", "2")]);
    let err = validate(&doc_with(c), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::MissingAttribute { .. }));
}

#[test]
fn image_missing_alt_errors() {
    let c = comp("artifact.image", &[("id", "img")]);
    let err = validate(&doc_with(c), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::MissingAttribute { .. }));
}

#[test]
fn json_depth_out_of_bounds_errors() {
    let c = comp("artifact.json", &[("id", "x"), ("depth", "9")]);
    let err = validate(&doc_with(c), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::AttrBound { .. }));
}

#[test]
fn json_collapsed_type_errors() {
    let c = comp("artifact.json", &[("id", "x"), ("collapsed", "maybe")]);
    let err = validate(&doc_with(c), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::AttrType { .. }));
}

#[test]
fn link_download_type_errors() {
    let c = comp("artifact.link", &[("id", "x"), ("download", "yes")]);
    let err = validate(&doc_with(c), Some(&Limits::default())).unwrap_err();
    assert!(matches!(err, ValidateError::AttrType { .. }));
}

#[test]
fn repo_viewers_are_accepted() {
    let c = comp("repo.code", &[("path", "src/lib.rs")]);
    validate(&doc_with(c), Some(&Limits::default())).expect("repo viewers are accepted with minimal checks");
}
