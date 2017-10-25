
error_chain! {
    errors {
        Unknown(req: ::reqwest::RequestBuilder, resp: ::reqwest::Response)
        Gql(errs: Vec<::json::Value>)
    }

    foreign_links {
        Reqwest(::reqwest::Error);
    }
}
