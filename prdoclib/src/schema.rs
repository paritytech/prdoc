use regex::Regex;
pub const JSON_SCHEMA: &'static str = include_str!("../../schema.json");

pub fn get_schema(strip_comments: bool) -> String {
    if !strip_comments {
        JSON_SCHEMA.to_string()
    }   else {
        let re = Regex::new(r"(?m)^//(.*)$").unwrap();
        let result = re.replace_all(JSON_SCHEMA, "");
        result.to_string().trim().to_string()
    }
}
