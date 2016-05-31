use rustc_serialize::json;

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct UserLoginBody {
    pub profile: String,
    pub state: String,
}

impl Into<String> for UserLoginBody {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct UserLogin {
    pub userLogin: UserLoginBody,
}

impl Into<String> for UserLogin {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}
