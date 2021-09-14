use select::document::Document;
use select::predicate::{Name,Predicate};
use crate::parser;
use crate::od::olaindex::{OLAINDEX, OlaindexExtras};
use crate::od::ODMethod;

/// Parses the OLAINDEX HTML Document type ods
fn olaindex_document(res:&str) -> Vec<String>{
     Document::from(res)
         //Find all <a data-route> tags
         .find(Name("a").or(Name("li")))
         .filter_map(|node|{
             let link = match node.attr("href"){
                 Some(link) => link,
                 None=> ""
             };
             if link.contains("?page="){
                 node.attr("href")
             }else{
                 node.attr("data-route")
             }
    }).filter(|link| {
         let mut paths:Vec<&str> = link.split("/").collect();
         !OLAINDEX::has_extra_paths(&mut paths,OlaindexExtras::ExcludeHomeAndDownload)
     })
         .filter(|link| !link.contains("javascript:void"))
         .map(|link| parser::sanitize_url(link)).collect()
}
/// Parses the usual HTML Document type ods
fn generic_document(res:&str) -> Vec<String>{
    Document::from(res)
        //Find all <a> tags
        .find(Name("a"))
        .filter_map(|node|{
        node.attr("href")
    }).filter(|link| {
        let mut paths:Vec<&str> = link.split("/").collect();
        !OLAINDEX::has_extra_paths(&mut paths,OlaindexExtras::ExcludeHomeAndDownload)
    }).filter(|link| !link.contains("javascript:void"))
        .map(|link| parser::sanitize_url(link)).collect()
}
/// Switch to a different way to parse Document type
pub fn filtered_links(res:&str, od_type:&ODMethod) ->Vec<String>{
    match od_type {
        ODMethod::OLAINDEX =>
            olaindex_document(res),
        _ => generic_document(res)
    }
}