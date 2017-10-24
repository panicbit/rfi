use std::time::{SystemTime,Duration,UNIX_EPOCH};
use reqwest::{self,Method,RequestBuilder};
use reqwest::header::{UserAgent,Accept,Authorization};

pub use self::token::Token;
pub use self::errors::*;

mod token;
mod errors;

const ROOT: &str = "https://api.github.com";
const USER_AGENT: &str = "gh: panicbit/rfi";

pub struct Client {
    token: Token,
    client: reqwest::Client,
}

impl Client {
    pub fn new(token: Token) -> Client {
        let client = reqwest::Client::new();

        Client {
            token,
            client,
        }
    }

    pub fn authenticated_request<S: AsRef<str>>(&self, method: Method, path: S) -> RequestBuilder {
        let url = format!("{}/{}", ROOT, path.as_ref());
        let mut request = self.client.request(method, &url);

        request
            .header(UserAgent::new(USER_AGENT))
            .header(Accept(vec!["application/vnd.github.v3+json".parse().unwrap()]))
            .header(Authorization(self.token.clone()));
        
        request
    }

    pub fn get_issue(&self, owner: &str, repo: &str, number: u32) -> Result<Issue> {
        let path = format!("repos/{owner}/{repo}/issues/{number}",
            owner = owner,
            repo = repo,
            number = number,
        );
        let mut req = self.authenticated_request(Method::Get, path);
        let mut resp = req.send()?;
        // use std::io::Read;
        // let mut body = String::new();
        // resp.read_to_string(&mut body).unwrap();
        // println!("{}", body);

        let issue = resp.json::<Issue>()
            .chain_err(|| ErrorKind::Unknown(req, resp))?;

        Ok(issue)
    }

    // pub fn get_pull_request(&self, owner: &str, repo: &str, number: u32) -> Result<PullRequest> {
    //     let path = format!("repos/{owner}/{repo}/pulls/{number}",
    //         owner = owner,
    //         repo = repo,
    //         number = number,
    //     );
    //     let mut req = self.authenticated_request(Method::Get, path);
    //     let mut resp = req.send()?;

    //     println!("{:#?}", resp);

    //     let pull_request = resp.json::<PullRequest>()
    //         .chain_err(|| ErrorKind::Unknown(req, resp))?;

    //     Ok(pull_request)
    // }
}

#[derive(Deserialize,Debug)]
pub struct Issue {
    number: u32,
    title: String,
    state: State,
    url: String,
    pull_request: Option<PullRequest>,
}

impl Issue {
    pub fn is_pull_request(&self) -> bool {
        self.pull_request.is_some()
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn number(&self) -> u32 {
        self.number
    }
}

#[derive(Deserialize,Debug)]
struct PullRequest {}

#[derive(Deserialize,Debug,PartialEq,Copy,Clone)]
#[serde(rename_all="lowercase")]
pub enum State {
    Open,
    Closed,
    Merged,
}

pub struct RateLimit {
    pub limit: u32,
    pub remaining: u32,
    pub reset: SystemTime,
}

fn reset_time_from_secs(secs: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(secs)
}
