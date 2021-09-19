use url::Url;
use regex::Regex;
use lazy_static::lazy_static;
use std::cmp::PartialEq;

lazy_static!{
    static ref LAST_SLASH_REG:Regex = Regex::new(r"/$").unwrap();
    static ref FILE_EXT_REG:Regex = Regex::new(r"/[a-zA-Z0-9\*~\+\-%\?\[\]\$_\.!‘\(\)= ]+\.[\w]{2,4}/?$").unwrap();
    static ref QUERY_REG:Regex = Regex::new(r"/\?\w+=\w+/").unwrap();
    static ref QUERY_PATH_REG:Regex = Regex::new(r"/\?/").unwrap();
}
#[derive(Debug)]
pub struct File {
    pub link:String,
    pub name:String,
    pub short_name:Option<String>,
    pub ext:Option<String>,
    pub dir_path:String
}
impl PartialEq for File{
    fn eq(&self, other: &Self) -> bool {
        self.link == other.link
    }
    fn ne(&self, other: &Self) -> bool {
        self.link != other.link
    }
}
impl File{
    pub fn new(link: &str) -> File{
        let name = File::retrieve_name(link).unwrap_or(String::from("untitled"));
        let ext = File::part_of_name(&name, true);
        let short_name = File::part_of_name(&name, false);
        let dir_path = File::dir_path(link);
        File {
            link:link.to_string(),
            name,
            ext,
            short_name,
            dir_path
        }
    }
    /// Retrieve the parent path(directories) of the file
    fn dir_path(link:&str) -> String{
        let no_query_url = QUERY_PATH_REG.replace(link,"/");
        let url = Url::parse(no_query_url.as_ref()).unwrap();

        let path = url.path();

        let dir_path = FILE_EXT_REG.replace(path,"/");

        dir_path.to_string()
    }
    /// Removes a path that starts a question mark. '/?/'
    fn query_check(url:&str) -> Option<String>{
        if QUERY_REG.is_match(url){
            let new_url = QUERY_REG.replace(url,"/");
            let url = Url::parse(new_url.as_ref()).unwrap();
            let paths = url.path_segments().unwrap();
            let path = paths.last().unwrap();
            Some(path.to_string())
        }else{
            None
        }
    }
    /// Retrieve the name of the file from the URL
    fn retrieve_name(url: &str) -> Option<String> {
        if let Some(name) = File::query_check(url){
            return Some(File::cut_name(name.as_str()));
        }

        let mut mut_url = Url::parse(url).unwrap();
        let immut_url = Url::parse(url).unwrap();

        if mut_url.path() == "/" {
            if let Some(query) = immut_url.query(){
                if query.starts_with("/"){
                    mut_url.set_query(None);
                    mut_url.set_path(query);
                }else{
                    return None;
                }
            }
        }
        let no_end_slash = LAST_SLASH_REG.replace(mut_url.path(),"").to_string();
        mut_url.set_path(no_end_slash.as_str());
        let url_paths =  mut_url.path_segments().ok_or_else(||"cannot as base").unwrap();

        match url_paths.last() {
            Some(name) => {
                if !name.is_empty(){
                    if name.ends_with("/") {
                        let mut name = String::from(name);
                        name.remove(name.len()-1);
                        Some(File::cut_name(name.as_str()))
                    }else{

                        Some(File::cut_name(name))
                    }
                }else{
                    None
                }

            },
            None => {
                None
            }
        }

    }
    /// Shortens the name of the file
    fn cut_name(name:&str) -> String{
        let file_limit= 160;
        if name.len() > file_limit {
            let start = name.len() - file_limit;
            name[start..name.len()].to_string()
        }else{
            name.to_string()
        }
    }
    /// Get the file extension
    pub fn part_of_name(name:&str, get_ext:bool) -> Option<String>{
        let name_split:Vec<&str> = name.split('.').collect();
        if name_split.len() == 2{
            if get_ext{
                Some(String::from(name_split[1]))
            }else{
                Some(String::from(name_split[0]))
            }
        }else{
            None
        }
    }
}