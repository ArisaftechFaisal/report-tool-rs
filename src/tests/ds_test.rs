use super::*;

#[test]
fn data_length() {
    let ds = get_test_ds();
    assert_eq!(ds.data.records.len(), 300);
}

#[test]
fn meta_custom_fields_length() {
    let ds = get_test_ds();
    // 23 in total, 2 html (ignored)
    assert_eq!(ds.meta.custom_fields.len(), 21);
}
