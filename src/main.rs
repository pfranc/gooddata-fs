#![deny(warnings)]
#![allow(non_snake_case)]
#[allow(unused_imports)]

extern crate cookie;
extern crate core;
extern crate env_logger;
extern crate hyper;
extern crate rustc_serialize;

use cookie::CookieJar;
use hyper::client::Client;
use hyper::header::{Accept, Cookie, ContentType, SetCookie, UserAgent, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use rustc_serialize::json;
use std::io::Read;
// use std::thread;

#[derive(RustcDecodable, RustcEncodable)]
struct PostUserLoginBody {
    login: String,
    password: String,
    remember: bool,
}

#[derive(RustcDecodable, RustcEncodable)]
struct PostUserLogin {
    postUserLogin: PostUserLoginBody,
}

// {"userLogin":{"profile":"/gdc/account/profile/3cf6a27afeed76b55caedf292691ac8a","state":"/gdc/account/login/3cf6a27afeed76b55caedf292691ac8a"}}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct UserLoginBody {
    profile: String,
    state: String,
}

#[derive(RustcDecodable, RustcEncodable)]
struct UserLogin {
    userLogin: UserLoginBody,
}

struct GoodDataClient {
    client: Client,
    server: String,
    jar: CookieJar<'static>,
    user: String,
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
impl GoodDataClient {
    /// Create Instance of GoodData Client
    fn new() -> GoodDataClient {
        GoodDataClient {
            client: Client::new(),
            server: "https://secure.gooddata.com".to_string(),
            jar: CookieJar::new(b"f8f9eaf1ecdedff5e5b749c58115441e"),
            user: String::new(),
        }
    }

    fn drop(&mut self) {
        println!("NOTE: Logging out is not implemented yet!");
    }

    /// Get Projects
    fn projects(&mut self) -> String {
        let mut res = self.get("/gdc/md");
        return self.get_content(&mut res);
    }

    /// Login to GoodData platform
    fn connect<S: Into<String>>(&mut self, username: S, password: S) {
        let payload = PostUserLogin {
            postUserLogin: PostUserLoginBody {
                login: username.into(),
                password: password.into(),
                remember: false,
            },
        };

        let mut raw = self.post("/gdc/account/login".to_string(),
                                json::encode(&payload).unwrap());

        self.refresh_token();

        let content = self.get_content(&mut raw);

        let user: UserLogin = json::decode(&content[..]).unwrap();
        let uri = user.userLogin.profile;

        let mut raw = self.get(uri);
        let rawUser = self.get_content(&mut raw);

        self.user = rawUser;
    }

    /// HTTP Method GET Wrapper
    fn get<S: Into<String>>(&mut self, path: S) -> hyper::client::response::Response {
        let uri = format!("{}{}", self.server, path.into());
        let mut res = self.client
            .get(&uri[..])
            .header(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])))
            .header(Accept(vec![
                             qitem(Mime(TopLevel::Application, SubLevel::Json,
                             vec![(Attr::Charset, Value::Utf8)])),
            ]))
            .header(UserAgent(GoodDataClient::user_agent().to_owned()))
            .header(Cookie::from_cookie_jar(&self.jar))
            .send()
            .unwrap();
        assert_eq!(res.status, hyper::Ok);
        println!("{:?}", res);

        self.print_response(&mut res);
        self.update_cookie_jar(&res);

        return res;
    }

    /// HTTP Method POST Wrapper
    fn post<S: Into<String>>(&mut self, path: S, body: S) -> hyper::client::response::Response {
        let uri = format!("{}{}", self.server, path.into());
        let payload = body.into();

        let mut res = self.client
            .post(&uri[..])
            .header(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])))
            .header(UserAgent(GoodDataClient::user_agent().to_owned()))
            .header(Accept(vec![
                            qitem(Mime(TopLevel::Application, SubLevel::Json,
                            vec![(Attr::Charset, Value::Utf8)])),
            ]))
            .body(&payload[..])
            .send()
            .unwrap();
        assert_eq!(res.status, hyper::Ok);

        self.print_response(&mut res);
        self.update_cookie_jar(&res);

        return res;
    }

    /// Get HTTP Response body
    fn get_content(&mut self, res: &mut hyper::client::Response) -> String {
        let mut buf = String::new();
        match res.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(_) => panic!("I give up."),
        };

        return buf;
    }

    /// Print HTTP Response
    fn print_response(&mut self, res: &mut hyper::client::Response) {
        return;

        let obj = res;

        println!("{:?}", obj);

        let content = self.get_content(obj);
        // let json = json::decode::<String>(&content[..]);
        println!("{}", content);
    }

    /// Update Cookies in Jar from HTTP Response
    fn update_cookie_jar(&mut self, res: &hyper::client::Response) {
        for setCookie in res.headers.get::<SetCookie>().iter() {
            for cookie in setCookie.iter() {
                // println!("{:?}", cookie);
                self.jar.add(cookie.clone());
            }
        }
    }

    /// Refresh GoodData TT (Temporary Token)
    fn refresh_token(&mut self) {
        // Refresh token
        self.get("/gdc/account/token");
    }

    fn user(&mut self) -> &String {
        &self.user
    }

    /// Construct User-Agent HTTP Header
    fn user_agent() -> String {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        return format!("gooddata-rust/{}", VERSION);
    }
}

fn main() {
    let mut gd = GoodDataClient::new();
    gd.connect("tomas.korcak+gem_tester@gooddata.com", "jindrisska");
    println!("{}", gd.user());
    println!("{}", gd.projects());
}
