enum Attribute {
    Field(DescriptorField),
    Label(DescriptorLabel),
    Timestamp(DescriptorTimestamp),
}

struct DescriptorTimestamp {
    name: String,
    format: String,
}

struct DescriptorLabel {
    name: String,
}

struct DescriptorField {
    name: String,
    data_type: String,
}
