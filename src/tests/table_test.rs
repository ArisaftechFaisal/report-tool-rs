use super::*;

#[test]
fn fkc_column_count_correct() {
    let fkc_table = get_test_ds().get_fkc_raw_table().unwrap();
    assert_eq!(fkc_table.cols.len(), 45);
}

#[test]
fn fkc_row_count_correct() {
    let fkc_table = get_test_ds().get_fkc_raw_table().unwrap();
    for col in fkc_table.cols.iter() {
        assert_eq!(col.contents.len(), 3100);
    }
}

#[test]
fn fkc_multiselect_str_correct() {
    let correct_str = r#"["【個人】小学校等の臨時休業に対応する保護者支援", "【個人・法人】企業主導型ベビーシッター利用者支援事業（特例措置）", "【個人・個人事業主】国民健康保険料（税）の減免に対する財政支援", "【個人】特別定額給付金", "【個人事業主・法人】小規模事業者持続化補助金（特別枠）"]"#;
    let fkc_table = get_test_ds().get_fkc_raw_table().unwrap();
    assert_eq!(fkc_table.cols[16].contents[12].as_str(), correct_str);
}

#[test]
fn fkc_multiselect_correct() {
    let correct_data: Vec<u8> = vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0];
    let correct_data = correct_data
        .into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>();
    let fkc_table = get_test_ds().get_fkc_raw_table().unwrap();
    for i in 17..=28 {
        assert_eq!(fkc_table.cols[i].contents[12], correct_data[i - 17]);
    }
}
