use regex::Regex;

#[extend::ext]
pub impl Regex {
    fn capture_first<'h>(&self, haystack: &'h str) -> Option<regex::Captures<'h>> {
        self.captures_iter(haystack).next()
    }
}
