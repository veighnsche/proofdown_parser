use proofdown_validate::schema_hint_for_kind;

#[test]
fn schema_hints_map_common_kinds() {
    assert_eq!(schema_hint_for_kind("api"), Some("api.schema.json"));
    assert_eq!(schema_hint_for_kind("dq"), Some("data_quality.schema.json"));
    assert_eq!(
        schema_hint_for_kind("coverage"),
        Some("coverage.schema.json")
    );
    assert_eq!(schema_hint_for_kind("a11y"), Some("a11y.schema.json"));
    assert_eq!(schema_hint_for_kind("e2e"), Some("e2e.schema.json"));
    assert_eq!(schema_hint_for_kind("unknown_kind"), None);
}
