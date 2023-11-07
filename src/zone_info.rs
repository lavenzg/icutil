pub struct ZoneInfo {
    pub version: String,
    pub zones: Vec<Zone>,
    pub names: Vec<String>,
    pub rules: Vec<Rule>,
    pub regions: Vec<String>,
}

pub enum Zone {
    AliasTo(u32),
    Detail {
        index: u32,
        trans_pre32: Vec<i32>,
        trans: Vec<i32>,
        trans_post32: Vec<i32>,
        type_offsets: Vec<i64>,
        type_map: Vec<u8>,
        final_rule_id: String,
        final_raw: u32,
        final_year: u32,
        aliases: Vec<u32>,
    }
}

pub struct Rule {
    pub id: String,
    pub values: Vec<i64>,
}