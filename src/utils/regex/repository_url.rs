use std::sync::LazyLock;

use regex::Regex;

pub static REPO_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https?://github\.com/(?<username>[^/]+?)/(?<reponame>[^/]+?)/").unwrap()
});

#[cfg(test)]
mod test {
    use crate::models::ext::regex::RegexExt as _;
    use crate::utils::regex::repository_url::REPO_URL_REGEX;

    #[test]
    fn test_repo_url_regex_username() {
        let res = REPO_URL_REGEX
            .capture_first("https://github.com/RustyNova016/cargo-berger/issues/51")
            .and_then(|cap| cap.name("username"))
            .map(|mat| mat.as_str());

        assert_eq!(res, Some("RustyNova016"))
    }

    #[test]
    fn test_repo_url_regex_reponame() {
        let res = REPO_URL_REGEX
            .capture_first("https://github.com/RustyNova016/cargo-berger/issues/51")
            .and_then(|cap| cap.name("reponame"))
            .map(|mat| mat.as_str());

        assert_eq!(res, Some("cargo-berger"))
    }
}
