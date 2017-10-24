
error_chain! {
    errors {
        UnexpectedFileNameFormat(file_name: String)
    }

    links {
        Github(::github::Error, ::github::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        Git(::git2::Error);
    }
}
