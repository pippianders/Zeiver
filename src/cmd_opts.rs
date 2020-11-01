use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt,Debug)]
#[structopt(name="Zeiver",about = "Scrape & download content from ODs.")]
pub struct Opts{

    ///Enable verbose output
    #[structopt(short,long)]
    pub verbose:bool,
    /// Specify the maximum depth for recursive scraping
    #[structopt(short = "d",long,default_value = "20")]
    pub depth:usize,
    /// Do not create directories from URL (download)
    ///
    /// Do not create a hierarchy of directories structured the same as the URL
    /// the file came from. All files will be saved to the current output directory instead.
    /// *Only available when downloading.
    #[structopt(long,parse(from_flag = std::ops::Not::not))]
    pub no_dirs:bool,
    /// Ignores a set of remote directories from being created
    ///
    /// Ignores a specified number of remote directories from being created.
    /// *Only available when downloading.
    /// Ex:
    /// URL: www.example.org/pub/xempcs/other/pics
    /// Original Save: ./pub/xempcs/other/pics
    /// After 2 cuts: ./other/pics
    #[structopt(short,long="cuts",default_value = "0")]
    pub cut_dirs:u32,
    /// Enables a request timeout (in secs)
    ///
    /// Adds a request timeout (in seconds). The timeout is applied from the time the request starts
    /// connecting until the response body has finished.
    #[structopt(short = "T",long)]
    pub timeout:Option<u64>,
    /// The waiting between each scrape & download request (secs)
    ///
    /// Wait a specified number of seconds between each scraping
    /// & download requests.
    #[structopt(short,long)]
    pub wait:Option<f32>,
    /// Save the links only
    ///
    /// After scraping, instead of downloading the files, save the links to them.
    /// *The downloader will be disabled when this option is active. Enables
    /// Recorder instead.
    #[structopt(long)]
    pub links_only:bool,
    /// The wait between each failed request (secs)
    ///
    /// Whenever a request fails, Zeiver will wait the specified
    /// number of seconds before retrying again
    #[structopt(long,default_value = "10")]
    pub retry_wait:f32,
    /// Wait a random amount of seconds between each request
    ///
    /// Randomly waits a specified number of seconds between each scraping
    /// & download requests. The time between requests will vary between
    /// 0.5 * [--wait,-w](inclusive) to 1.5 * [--wait,-w](exclusive)
    #[structopt(long)]
    pub random_wait:bool,
    /// The amount of times to retry a failed connection/request
    #[structopt(short,long,default_value = "20")]
    pub tries:u32,
    /// Maximum redirects to follow
    #[structopt(short,long,default_value = "10")]
    pub redirects:usize,
    /// Files to accept for download
    ///
    /// Using Regex, specify which files to accept for downloading.
    /// Only the files that matches the regex will be acceptable
    /// for download. (This option takes precedence over --reject, -r)
    #[structopt(short = "A",long)]
    pub accept:Option<String>,
    /// Files to reject for download
    ///
    /// Using Regex, specify which files to reject for downloading.
    /// Only the files that match the regex will be rejected
    /// for download. (--accept, -a takes precedence over this option)
    #[structopt(short = "R",long,conflicts_with = "accept")]
    pub reject:Option<String>,
    /// The User Agent header to use
    #[structopt(short="U")]
    pub user_agent:Option<String>,
    /// Sets the default headers 'header:value'
    ///
    /// Sets the default headers for every request. Must use the
    /// 'header$value' format. Each header must also be separated by a comma.
    /// Ex: -H content-length$128,"accept$ text/html, application/xhtml+xml, image/webp"
    #[structopt(short="H",long,use_delimiter(true))]
    pub headers:Option<Vec<String>>,
    /// The proxy to use
    #[structopt(long)]
    pub proxy:Option<String>,
    /// Authentication for the proxy 'username:password'
    ///
    /// The basic authentication needed to use the proxy. Must use the
    /// 'username:password' format.
    #[structopt(long)]
    pub proxy_auth:Option<String>,
    /// Read URLs from a local or external file
    ///
    /// Read URLs from a file. *Each URL is read line by line.
    #[structopt(short,long,requires_if("None","urls"))]
    pub input_file: Option<PathBuf>,
    /// Save file location
    ///
    /// The local file path to save downloading files.
    /// Ex: ./downloads/images/dir
    #[structopt(short,long,default_value = "./")]
    pub output:PathBuf,

    /// The URLs to download content from
    #[structopt(name = "URLS",required_unless("input-file"))]
    pub urls:Vec<PathBuf>,
}
impl Opts{
    pub fn new() -> Opts{
        Opts::from_args()
    }
}