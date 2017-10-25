use std::time::{SystemTime,Duration,UNIX_EPOCH};
use reqwest::{self,Method,RequestBuilder,Body};
use reqwest::header::{UserAgent,Accept,Authorization};

pub use self::token::Token;
pub use self::errors::*;

use serde::Serialize;

mod token;
mod errors;

const ROOT: &str = "https://api.github.com/graphql";
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

    pub fn authenticated_request<Q: AsRef<str>, V: Serialize>(&self, query: Q, variables: V) -> RequestBuilder {
        #[derive(Serialize)]
        struct GqlQuery<'a, V: Serialize> {
            query: &'a str,
            variables: V,
        }

        let mut request = self.client.post(ROOT);

        request
            .header(UserAgent::new(USER_AGENT))
            .header(Accept(vec!["application/vnd.github.v4+json".parse().unwrap()]))
            .header(Authorization(self.token.clone()))
            .json(&GqlQuery { query: query.as_ref(), variables });
        
        request
    }

    pub fn get_issue(&self, owner: &str, repo: &str, number: u32) -> Result<Issue> {
        #[derive(Serialize)]
        struct Variables<'a> {
            owner: &'a str,
            repo: &'a str,
            number: u32,
        }

        let mut req = self.authenticated_request(r#"
            query($owner: String!, $repo: String!, $number: Int!) {
                repository(owner: $owner, name: $repo) {
                    issue: issueOrPullRequest(number: $number) {
                        ...on PullRequest { number title state url }
                        ... on Issue { number title state url }
                    }
                }
            }
        "#, Variables { owner, repo, number });

        let mut resp = req.send()?;

        // use std::io::Read;
        // let mut body = String::new();
        // resp.read_to_string(&mut body).unwrap();
        // println!("{}", body);

        let gql_resp = resp.json::<GqlResponse<GqlRepository<GqlIssue>>>()
            .chain_err(|| ErrorKind::Unknown(req, resp))?;

        match gql_resp {
            GqlResponse::Errors { errors } => bail!(ErrorKind::Gql(errors)),
            GqlResponse::Data { data } => Ok(data.repository.issue)
        }
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

#[derive(Deserialize,Serialize,Debug)]
pub struct Issue {
    number: u32,
    title: String,
    state: State,
    url: String,
}

impl Issue {
    pub fn state(&self) -> State {
        self.state
    }

    pub fn number(&self) -> u32 {
        self.number
    }
}

#[derive(Deserialize,Serialize,Debug,PartialEq,Copy,Clone)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
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

#[derive(Deserialize)]
#[serde(untagged)]
enum GqlResponse<T> {
    Errors { errors: Vec<::json::Value> },
    Data { data: T },
}

#[derive(Deserialize)]
struct GqlRepository<T> {
    repository: T,
}

#[derive(Deserialize)]
struct GqlIssue {
    issue: Issue,
}
