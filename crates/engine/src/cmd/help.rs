//! Help → project links (IrisVector's GitHub repository and the project it is based on). The
//! commands return the URL; the frontend opens it.
//!
//! IrisVector is a fork of VectorCraft for the Iris ecosystem: no ArtCraft community links here
//! (their brand terms ask forks not to use them). `UPSTREAM_URL` credits the original project.

use serde_json::{Value, json};

use super::{CommandSpec, Result, always, cmd};

/// IrisVector's source repository: code, issues and releases.
pub const GITHUB_URL: &str = "https://github.com/GabrielAlvesB/IrisVector";
/// The project IrisVector is based on.
pub const UPSTREAM_URL: &str = "https://github.com/storytold/vectorcraft";

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.github", "IrisVector on GitHub", ["Help"], None, "{} → {url} source code, issues and releases", always, |_, _| url(GITHUB_URL)),
        cmd!(query "help.upstream", "Based on VectorCraft", ["Help"], None, "{} → {url} the project IrisVector is based on", always, |_, _| url(UPSTREAM_URL)),
        cmd!(query "help.links", "Links", [], None, "{} → {github, upstream}", always, |_, _| Ok(json!({ "github": GITHUB_URL, "upstream": UPSTREAM_URL }))),
    ]
}

fn url(u: &str) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/GabrielAlvesB/IrisVector");
        assert_eq!(s.execute("help.upstream", &json!({})).unwrap()["url"], "https://github.com/storytold/vectorcraft");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["github"], "https://github.com/GabrielAlvesB/IrisVector");
        assert!(s.execute("help.discord", &json!({})).is_err(), "no ArtCraft community links");
    }
}
