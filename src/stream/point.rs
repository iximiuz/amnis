pub struct Point;

impl Point {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum Attribute {
    Field(DescriptorField),
    Label(DescriptorLabel),
    Timestamp(DescriptorTimestamp),
}

pub struct DescriptorTimestamp {
    name: String,
    format: String,
}

pub struct DescriptorLabel {
    name: String,
}

pub struct DescriptorField {
    name: String,
    data_type: String,
}

impl DescriptorField {
    pub fn new(name: String, data_type: String) -> Self {
        Self { name, data_type }
    }
}
