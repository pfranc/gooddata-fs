use std::collections::HashMap;
use rustc_serialize::json;

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct AccountSettingBody {
    pub country: Option<String>,
    pub firstName: Option<String>,
    pub language: Option<String>,
    pub ssoProvider: Option<String>,
    pub timezone: Option<String>,
    pub position: Option<String>,
    pub authenticationModes: Option<Vec<String>>,
    pub companyName: Option<String>,
    pub login: Option<String>,
    pub email: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub lastName: Option<String>,
    pub phoneNumber: Option<String>,
    pub links: Option<HashMap<String, String>>,
}

impl Into<String> for AccountSettingBody {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct AccountSetting {
    pub accountSetting: AccountSettingBody,
}

impl Into<String> for AccountSetting {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}
