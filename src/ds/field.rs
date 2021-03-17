#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum FieldType {
    Custom(usize),
    Computed(ComputedFieldType),
    Id,
    UserId,
    CreatedAt,
    Gender,
    Job,
    Prefecture,
    Region,
    MaritalStatus,
    Children,
    MaritalStatusAndChildren,
    YearlyIncome,
    Age,
    AgeGroup,
    AgeGroup1060,
    AgeGroup1070,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ComputedFieldType {
    // Age grouping (10 ~ 60)
    AgeGroup1060AggregateLabel,
    AgeGroup1060AggregateValue,
    AgeGroup1060AggregateDisplay,
    // Age grouping (10 ~ 70)
    AgeGroup1070AggregateLabel,
    AgeGroup1070AggregateValue,
    AgeGroup1070AggregateDisplay,
    // Gender grouping
    GenderAggregateLabel,
    GenderAggregateValue,
    GenderAggregateDisplay,
    // Marital status grouping
    MaritalStatusAggregateLabel,
    MaritalStatusAggregateValue,
    MaritalStatusAggregateDisplay,
    // Children grouping
    ChildrenAggregateLabel,
    ChildrenAggregateValue,
    ChildrenAggregateDisplay,
    // Job grouping
    JobAggregateLabel,
    JobAggregateValue,
    JobAggregateGraphLabel,
    JobAggregatePercentage,
    // Region grouping
    RegionAggregateLabel,
    RegionAggregateValue,
    RegionAggregateGraphLabel,
    RegionAggregatePercentage,
    // Income range grouping
    YearlyIncomeAggregateLabel,
    YearlyIncomeAggregateValue,
    YearlyIncomeAggregateGraphLabel,
    YearlyIncomeAggregatePercentage,

    // Custom
    Custom(usize),
}
