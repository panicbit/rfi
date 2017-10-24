use std::path::Path;
use git2::Repository;
use git2::build::CheckoutBuilder;
use std::fs::{self, File};
use std::io::Read;
use regex::Regex;
use errors::*;
use github;

#[derive(Debug)]
pub struct Rfc {
    pub number: u32,
    pub short_title: String,
    pub issues: Vec<github::Issue>,
}

impl Rfc {
    pub fn from_path<P: AsRef<Path>>(path: P, github: &github::Client) -> Result<Rfc> {
        use self::ErrorKind::UnexpectedFileNameFormat;
        lazy_static! {
            static ref FILE_NAME_RE: Regex = Regex::new(r"^(\d+)-(.*)\.md$").unwrap();
        }
        let path = path.as_ref();
        let mut file = File::open(path)?;
        // `expect` should be fine if `File` opening succeeds
        let file_name = path.file_name().expect("file name").to_string_lossy().into_owned();
        let caps = FILE_NAME_RE.captures(&file_name)
            .ok_or_else(|| UnexpectedFileNameFormat(file_name.to_string()))?;
        let number = caps[1].parse().chain_err(|| UnexpectedFileNameFormat(file_name.to_string()))?;

        let mut markdown = String::new();
        file.read_to_string(&mut markdown)?;
        let issues = Self::parse_issues(&markdown);

        // println!("found tickets: {:#?}", issues);

        let issues = Self::get_github_tickets(github, issues)
            .chain_err(|| format!("Couldn't fetch ticket info for RFC {}", number))?;

        Ok(Rfc {
            number,
            short_title: caps[2].to_string(),
            issues: issues,
        })
    }

    fn parse_issues(data: &str) -> Vec<Issue> {
        use ::pulldown_cmark::{Parser,Event,Tag};
        let mut parser = Parser::new(data).peekable();
        let mut tickets = Vec::new();

        // Skip header if it's the very first item
        if let Some(&Event::Start(Tag::Header(_))) = parser.peek() {
            parser.next();
        }

        lazy_static! {
            static ref RUST_ISSUE_RE: Regex = Regex::new(r"(?i)^Rust Issue").unwrap();
        }

        let items = parser
            .skip_while(|ev| !matches!(*ev, Event::Text(ref text) if RUST_ISSUE_RE.is_match(text)))
            .take_while(|ev| !matches!(*ev, Event::Start(Tag::Header(_))));

        for item in items {
            match item {
                Event::Text(text) => {
                    // println!("Text: {}", text);

                    tickets.extend(Issue::from_text(&text));
                }
                Event::Start(Tag::Link(link,..)) => {
                    // println!("Link: {}", link);
                    tickets.extend(Issue::from_text(&link));
                }
                _ => {} //println!("{:?}", item)
            }
        }

        tickets
    }

    fn get_github_tickets(github: &github::Client, issues: Vec<Issue>) -> Result<Vec<github::Issue>> {
        issues
        .into_iter()
        .map(|i| github.get_issue(&i.owner, &i.repo, i.number).map_err(Error::from))
        .collect()
    }
}

#[derive(Debug)]
pub struct Issue {
    pub owner: String,
    pub repo: String,
    pub number: u32,
}

impl Issue {
    fn from_text<'a>(text: &'a str) -> Box<Iterator<Item=Issue> + 'a> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"https://github.com/(?P<owner>[^/]+)/(?P<repo>[^/]+)/(issues|pull)/(?P<number>\d+)").unwrap();
        }

        Box::new(RE.captures_iter(text).map(|caps| Issue {
            owner: caps["owner"].into(),
            repo: caps["repo"].into(),
            number: caps["number"].parse().unwrap(),
        }))
    }
}

pub fn get_all<'a, P: AsRef<Path>>(path: P, github: &'a github::Client) -> Result<Box<Iterator<Item=Result<Rfc>> + 'a>> {
    let path = path.as_ref();

    get_repo(path)?;

    let rfcs = fs::read_dir(path.join("text"))?
        .map(move |entry| -> Result<_> {
            let entry = entry?;

            if !entry.file_type()?.is_dir() {
                let rfc = Rfc::from_path(entry.path(), github)?;

                Ok(Some(rfc))
            } else {
                Ok(None)
            }
        })
        .filter_map(|res| match res {
            Ok(None) => None,
            Ok(Some(val)) => Some(Ok(val)),
            Err(e) => Some(Err(e))
        });

    Ok(Box::new(rfcs))
}

fn get_repo<P: AsRef<Path>>(path: P) -> Result<Repository> {
    const RFC_REPO_URL: &str = "https://github.com/rust-lang/rfcs";

    if path.as_ref().exists() {
        let repo = Repository::open(path)?;

        update_repo(&repo)?;

        Ok(repo)
    }
    else {
        let repo = Repository::clone(RFC_REPO_URL, path)?;
        
        Ok(repo)
    }
}

fn update_repo(repo: &Repository) -> Result<()> {
    let mut origin = repo.find_remote("origin")?;

    origin.fetch(&["master"], None, None)?;
    repo.set_head("refs/remotes/origin/master")?;
    repo.checkout_head(Some(CheckoutBuilder::new().force()))?;

    Ok(())
}
