
error_chain! {
    errors {
        Unknown(req: ::reqwest::RequestBuilder, resp: ::reqwest::Response)
    }

    foreign_links {
        Reqwest(::reqwest::Error);
    }
}
