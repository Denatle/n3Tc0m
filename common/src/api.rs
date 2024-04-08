use reqwest::Body;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct DataString {
    pub string: String,
}

impl From<DataString> for Body {
    fn from(data: DataString) -> Body {
        serde_json::to_string(&data).unwrap().into()
    }
}

