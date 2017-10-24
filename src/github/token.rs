use std::env;
use std::fmt;
use std::str::FromStr;
use std::ffi::OsStr;
use reqwest::header::Scheme;

#[derive(Clone)]
pub struct Token(String);

impl Token {
    pub fn from_env_var<S: AsRef<OsStr>>(name: S) -> Result<Token, env::VarError> {
        env::var(name).map(Token)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Token").finish()
    }
}

impl FromStr for Token {
    type Err = &'static str;

    fn from_str(_token: &str) -> Result<Token, &'static str> {
        Err("Cannot parse github token")
    }
}

impl Scheme for Token {
    fn scheme() -> Option<&'static str> {
        Some("token")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
