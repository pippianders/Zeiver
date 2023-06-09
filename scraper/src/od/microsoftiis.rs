use super::all;
use crate::parser;
use select::document::Document;
use select::predicate::{Name, Or, Predicate};

const IDENTIFIER_PARENT: &str = "[To Parent Directory]";
const IDENTIFIER_DIR: &str = "<dir>";
pub const IDENTIFIER: &str = "Microsoft-IIS";
pub struct MicrosoftIIS;
impl MicrosoftIIS {
    pub fn is_od(res: &str, server_name: &str) -> bool {
        MicrosoftIIS::parent_id(res)
            || MicrosoftIIS::directory_id(res)
            || MicrosoftIIS::header_id(server_name)
    }
    /// Look for unique parent directory text `[Parent Directory]`
    fn parent_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("pre").descendant(Name("a")))
            .any(|node| node.text() == IDENTIFIER_PARENT)
    }
    /// Look for unique directory tag `<dir>`
    fn directory_id(res: &str) -> bool {
        Document::from(res)
            .find(Name("pre"))
            .any(|node| node.text().contains(IDENTIFIER_DIR))
    }
    /// Identify by looking at Server header
    fn header_id(server_name: &str) -> bool {
        server_name.contains(IDENTIFIER)
    }
    /// Parses Microsoft-IIS HTML Documents
    pub fn search(res: &str, url: &str) -> Vec<String> {
        Document::from(res)
            .find(Or(
                Name("pre").descendant(Name("a")),
                Name("tr").descendant(Name("td").descendant(Name("a"))),
            ))
            .filter(|node| all::no_parent_dir(url, &node.text(), node.attr("href")))
            .filter_map(|node| node.attr("href"))
            .filter(|link| !link.contains("javascript:"))
            .map(|link| parser::sanitize_url(link))
            .collect()
    }
}
