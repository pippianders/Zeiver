use select::document::Document;
use select::predicate::{Name, Class,Attr, Predicate};
use crate::parser;
use crate::od::olaindex::{OLAINDEX, OlaindexExtras};
use crate::od::apache::Apache;
use crate::od::directory_listing_script::DirectoryListingScript;
use crate::od::ODMethod;

/// Parses the Evoluted Directory Listing Script HTML Document type ods
fn directory_listing_script_document(res:&str,url:&str) -> Vec<String>{
    Document::from(res)
        //Find all <a> tags
        .find(Attr("id","listingcontainer").descendant(Name("a"))
            .or(Class("table-container").descendant(Name("a"))))
        .filter(|node| no_parent_dir(url,&node.text(),node.attr("href")))
        .filter_map(|node| {
            node.attr("href")
        }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}
/// Parses the nginx HTML Document type ods
/// Shares qualities with apache
fn nginx_document(res: &str,url:&str) -> Vec<String> {
    Document::from(res).find(
        Name("a")
    ).filter(|node| no_parent_dir(url,&node.text(),node.attr("href")))
        .filter(|node| !Apache::has_extra_query(node.attr("href").unwrap()))
        .filter_map(|node| {
            node.attr("href")
        }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}

/// Parses the Apache HTML Document type ods
fn apache_document(res: &str,url:&str) -> Vec<String> {
    Document::from(res).find(
        Name("tr").descendant(Name("td").descendant(Name("a")))
            .or(Name("pre").descendant(Name("a")))
            .or(Name("li").descendant(Name("a")))
    ).filter(|node| no_parent_dir(url,&node.text(),node.attr("href")))
        .filter(|node| !Apache::has_extra_query(node.attr("href").unwrap()))
        .filter_map(|node| {
            node.attr("href")
        }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}

/// Parses the Directory Lister HTML Document type ods
fn directory_lister_document(res: &str, url: &str) -> Vec<String> {
    Document::from(res)
        //Find all <a> tags
        .find(Name("ul").descendant(Name("a")))
        .filter(|node| {
            let link = node.attr("href").unwrap();
            !url.contains(link) && no_parent_dir(url,&node.text(),node.attr("href"))
        })
        .filter_map(|node| {
            node.attr("href")
        }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}

/// Parses the AutoIndex PHP HTML Document type ods
fn autoindex_document(res: &str,url:&str) -> Vec<String> {
    Document::from(res)
        //Find all <a> tags
        .find(Name("tbody").descendant(Class("autoindex_a").or(Class("default_a"))))
        .filter(|node| no_parent_dir(url,&node.text(),node.attr("href")))
        .filter_map(|node| {
            node.attr("href")
        }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}

/// Parses the OLAINDEX HTML Document type ods
fn olaindex_document(res: &str,url:&str) -> Vec<String> {
    Document::from(res)
        //Find all <a data-route> tags
        .find(Name("div")
            .and(Class("mdui-container").or(Class("container")))
            .descendant(Name("a").or(Name("li"))))
        .filter(|node| no_parent_dir(url,&node.text(),node.attr("href")))
        .filter_map(|node| {
            if node.attr("data-route").is_some() {
                node.attr("data-route")
            } else {
                node.attr("href")
            }
        })
        .filter(|link| {
            let mut paths: Vec<&str> = link.split("/").collect();
            !OLAINDEX::has_extra_paths(&mut paths, OlaindexExtras::ExcludeHomeAndDownload)
        }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}

/// Parses the usual HTML Document type ods
fn generic_document(res: &str,url:&str) -> Vec<String> {
    Document::from(res)
        //Find all <a> tags
        .find(Name("a"))
        .filter(|node| no_parent_dir(url,&node.text(),node.attr("href")))
        .filter_map(|node| {
            node.attr("href")
        }).filter(|link| {
        let mut paths: Vec<&str> = link.split("/").collect();
        !OLAINDEX::has_extra_paths(&mut paths, OlaindexExtras::ExcludeHomeAndDownload)
    }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}

/// Switch to a different way to parse Document type
pub fn filtered_links(res: &str, url: &str, od_type: &ODMethod) -> Vec<String> {
    match od_type {
        ODMethod::OLAINDEX => olaindex_document(res,url),
        ODMethod::AutoIndexPHP | ODMethod::AutoIndexPHPNoCrumb => autoindex_document(res,url),
        ODMethod::DirectoryLister => directory_lister_document(res, url),
        ODMethod::DirectoryListingScript => directory_listing_script_document(res,url),
        ODMethod::Apache => apache_document(res,url),
        ODMethod::NGINX => nginx_document(res,url),
        _ => generic_document(res,url)
    }
}

/// Check if link leads back to parent directory
fn no_parent_dir(url:&str, content: &str,href:Option<&str>) -> bool {
    //Check for `parent directory` phrase
    let not_parent_dir = !content.trim().to_lowercase().starts_with("parent directory");
    //Check for back paths
    let no_back_path = match href {
        Some(link) => link != "." && link != "../" && link != ".." && link != "./",
        None => false
    };
    //Check for `www.example.com/index.php?dir=`
    let no_home_navigator = match href{
        Some(link) => !DirectoryListingScript::is_home_navigator(link),
        None => false
    };
    //Check for URLs leading back to homepage
    let no_home_url = match href {
        Some(link) => {
            let mut new_link = parser::remove_http(link);
            new_link = parser::remove_last_slash(&new_link);
            let mut new_url = parser::remove_http(url);
            new_url = parser::remove_last_slash(&new_url);
            new_link != new_url
        },
        None => false
    };
    not_parent_dir && no_back_path && no_home_navigator && no_home_url
}

#[cfg(test)]
mod tests{
    use super::no_parent_dir;
    #[test]
    fn no_parent_test(){
        const HOME_URL:&str = "https://ftp.example.jp";
        assert_eq!(no_parent_dir(HOME_URL,"Parent directory/",Some("../")),false);
        assert_eq!(no_parent_dir(HOME_URL,"Parent Directory",Some("..")),false);
        assert_eq!(no_parent_dir(HOME_URL,"Parent Directory",Some("./")),false);
        assert_eq!(no_parent_dir(HOME_URL,"parent directory",Some(".")),false);
        assert_eq!(no_parent_dir(HOME_URL,"Carrots and java",Some("../")),false);
        assert_eq!(no_parent_dir(HOME_URL,"Carrots and java",Some("./")),false);
        assert_eq!(no_parent_dir(HOME_URL,"Carrots and java",Some("..")),false);
        assert_eq!(no_parent_dir(HOME_URL,"Carrots and java",Some(".")),false);
        assert_eq!(no_parent_dir(HOME_URL,"Carrots and java",Some("https://www.example.com/path/index.php?dir=")),false);
        assert_eq!(no_parent_dir(HOME_URL,"Drink Soda",Some("https://ftp.example.jp")),false);

        assert_eq!(no_parent_dir(HOME_URL,"Carrots and java",Some("./Carrots%20and%20java")),true);
        assert_eq!(no_parent_dir(HOME_URL,"Drink Soda",Some("Drink%20Soda")),true);
        assert_eq!(no_parent_dir(HOME_URL,"Carrots and java",Some("https://www.example.com/path/index.php?dir=Outboards%2F5-27")),true);
        assert_eq!(no_parent_dir(HOME_URL,"Drink Soda",Some("https://example.me")),true);

    }
}