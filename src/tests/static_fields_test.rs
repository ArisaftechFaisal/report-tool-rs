use super::*;

#[test]
fn id_str_correct() {
    test_static_field_str(
        0,
        FieldType::Id,
        "User-648906b7-5c6e-4fbf-baef-474bda703375_CampaignPost--1593770502732",
    );
}

#[test]
fn user_id_str_correct() {
    test_static_field_str(
        0,
        FieldType::UserId,
        "User-648906b7-5c6e-4fbf-baef-474bda703375",
    );
}

#[test]
fn created_at_str_correct() {
    test_static_field_str(0, FieldType::CreatedAt, "2020年7月3日 19:01");
}

#[test]
fn gender_str_correct() {
    test_static_field_str(0, FieldType::Gender, "女性");
    test_static_field_str(4, FieldType::Gender, "男性");
}

#[test]
fn prefecture_str_correct() {
    test_static_field_str(0, FieldType::Prefecture, "東京都");
    test_static_field_str(1, FieldType::Prefecture, "千葉県");
    test_static_field_str(3, FieldType::Prefecture, "滋賀県");
}

#[test]
fn region_str_correct() {
    test_static_field_str(0, FieldType::Region, "関東");
    test_static_field_str(4, FieldType::Region, "中部");
    test_static_field_str(8, FieldType::Region, "北海道");
}

#[test]
fn age_str_correct() {
    test_static_field_str(0, FieldType::Age, "24")
}

#[test]
fn age_group_str_correct() {
    test_static_field_str(0, FieldType::AgeGroup, "20代");
    test_static_field_str(4, FieldType::AgeGroup, "30代");
    test_static_field_str(5, FieldType::AgeGroup, "40代");
}

#[test]
fn age_group_1060_str_correct() {
    test_static_field_str(0, FieldType::AgeGroup1060, "20代");
    test_static_field_str(1, FieldType::AgeGroup1060, "10代以下");
    test_static_field_str(4, FieldType::AgeGroup1060, "30代");
    test_static_field_str(36, FieldType::AgeGroup1060, "60代以上");
    test_static_field_str(47, FieldType::AgeGroup1060, "50代");
}

#[test]
fn age_group_1070_str_correct() {
    test_static_field_str(0, FieldType::AgeGroup1070, "20代");
    test_static_field_str(1, FieldType::AgeGroup1070, "10代以下");
    test_static_field_str(39, FieldType::AgeGroup1070, "60代");
    test_static_field_str(220, FieldType::AgeGroup1070, "70代以上");
}

#[test]
fn job_str_correct() {
    test_static_field_str(0, FieldType::Job, "学生");
    test_static_field_str(3, FieldType::Job, "会社員（その他）");
}

#[test]
fn marital_status_str_correct() {
    test_static_field_str(0, FieldType::MaritalStatus, "未婚");
    test_static_field_str(3, FieldType::MaritalStatus, "既婚");
}

#[test]
fn children_str_correct() {
    test_static_field_str(0, FieldType::Children, "0人");
    test_static_field_str(3, FieldType::Children, "1人");
    test_static_field_str(10, FieldType::Children, "2人");
}

#[test]
fn marital_status_and_children_str_correct() {
    test_static_field_str(0, FieldType::MaritalStatusAndChildren, "未婚(子なし)");
    test_static_field_str(3, FieldType::MaritalStatusAndChildren, "既婚(子あり)");
    test_static_field_str(4, FieldType::MaritalStatusAndChildren, "既婚(子なし)");
    test_static_field_str(5, FieldType::MaritalStatusAndChildren, "未婚(子あり)");
}

#[test]
fn income_range_str_correct() {
    test_static_field_str(0, FieldType::YearlyIncome, "700～800万円未満");
    test_static_field_str(2, FieldType::YearlyIncome, "100万円未満");
}
