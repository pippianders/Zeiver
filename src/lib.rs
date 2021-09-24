use std::sync::Arc;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::time::Duration;
use reqwest;
use crawler;
use cmd_opts;
use crawler::WebCrawler;
use logger;

pub struct Zeiver;

impl Zeiver {
    /// Activates Zeiver
    pub async fn crawl() {
        let web_crawler = Arc::new(crawler::WebCrawler::new());
        let mut opts = cmd_opts::Opts::new();
        Zeiver::clean_urls_list(&mut opts);
        if !opts.urls.is_empty() {
            Zeiver::multi_thread(web_crawler, opts.urls).await;
        } else if opts.input_file.is_some() {
            let urls = crawler::WebCrawler::input_file_links(opts.input_file).await;
            Zeiver::multi_thread(web_crawler, urls).await;
        }else{
            web_crawler.recorder_file_task().await;
        }
    }
    /// Performs tasks given to the WebCrawler under multi-threading
    /// NOTE: The amount of threads depends on the amount of URLs specified
    /// by the user.
    async fn multi_thread(web_crawler: Arc<crawler::WebCrawler>, urls: Vec<PathBuf>,) {
        let client_builder = reqwest::Client::builder();
        let client = Arc::new(Zeiver::client_creator(client_builder).unwrap());
        for url in urls {
            Zeiver::establish_task(url,web_crawler.clone(),client.clone()).await;
        }
    }
    /// Execute the task provided by the commandline
    async fn establish_task(url:PathBuf,web_clone:Arc<WebCrawler>,client_clone:Arc<reqwest::Client>){
        let opts = cmd_opts::Opts::new();
        if opts.print_headers{
            WebCrawler::print_all_headers(&client_clone,url,
                                          opts.tries,opts.wait,opts.retry_wait,
                                          opts.random_wait,opts.verbose).await.unwrap();
        }else if opts.print_header.is_some(){
            WebCrawler::print_header(opts.print_header.unwrap(),&client_clone,url,
                                     opts.tries,opts.wait,opts.retry_wait,
                                     opts.random_wait,opts.verbose).await.unwrap();
        }else{
            Zeiver::spawn_thread(url,web_clone,client_clone,opts.record_only,opts.record,opts.test).await;
        }
    }
    /// Spawns a new thread
    async fn spawn_thread(url:PathBuf,web_clone:Arc<WebCrawler>,client_clone:Arc<reqwest::Client>,
                          record_only:bool,record: bool, debug: bool){
        tokio::spawn(async move {
            let scraper = web_clone.scraper_task(&client_clone, Some(url)).await;
            let arc_scraper = Arc::new(scraper);
            if !debug {
                let arc_count = Arc::strong_count(&web_clone) - 1;//Total amount of web crawlers sharing a pointer
                if record_only {
                    web_clone.recorder_task(arc_scraper, arc_count).await;
                } else {
                    let arc_scraper_clone = Arc::clone(&arc_scraper);
                    if record {
                        web_clone.recorder_task(arc_scraper_clone, arc_count).await;
                    }
                    web_clone.downloader_task(&client_clone, arc_scraper).await;
                }
            }
        }).await.unwrap();
    }
    // Adds configurations to the Client
    fn client_creator(builder: reqwest::ClientBuilder) -> Result<reqwest::Client, reqwest::Error> {
        let opts = cmd_opts::Opts::new();
        let mut builder = builder;
        // Proxy
        if opts.proxy.is_some() {
            let mut proxy = reqwest::Proxy::all(&opts.proxy.unwrap()).expect("Not a valid proxy!");
            if opts.proxy_auth.is_some() {
                let auth = opts.proxy_auth.unwrap();
                let mut auth_iter = auth.split("$");
                let user = auth_iter.next().expect("Username does not exist");
                let user = user.trim();
                let pass = auth_iter.next().expect("Password does not exist");
                let pass = pass.trim();
                proxy = proxy.basic_auth(user, pass);
            }
            builder = builder.proxy(proxy);
        }
        // Headers
        if opts.headers.is_some() {
            let mut header_map = reqwest::header::HeaderMap::new();
            let headers = opts.headers.unwrap();
            for header in headers {
                let mut head_arr = header.split("$");

                //Set Header Name
                let header_name_str = head_arr.next().expect("Header Name not found!").trim();
                let header_name_lower = header_name_str.to_lowercase();
                let header_name = reqwest::header::HeaderName::from_str(header_name_lower.as_str());
                let header_name = match header_name {
                    Ok(name) => name,
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                        continue;
                    }
                };

                //Set Header Value
                let header_value_str = head_arr.next().expect("Header Value not found!").trim();
                let header_value_lower = header_value_str.to_lowercase();
                let header_value = reqwest::header::HeaderValue::from_str(header_value_lower.as_str());
                let header_value = match header_value {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                        continue;
                    }
                };
                logger::log_split("Name",header_name.as_str());
                logger::log_split("Value",header_value.to_str().unwrap());
                logger::new_line();
                header_map.insert(header_name, header_value);
            }
            builder = builder.default_headers(header_map);
        }
        // Timeout
        if opts.timeout.is_some() {
            let secs = opts.timeout.unwrap();
            let duration = Duration::from_secs(secs);
            builder = builder.timeout(duration);
        }
        // User Agent
        let user_agent = match opts.user_agent {
            Some(agent) => agent,
            None => format!("Zeiver/{}", env!("CARGO_PKG_VERSION"))
        };
        builder = builder.user_agent(user_agent);
        // Accept all certificates
        builder = builder.danger_accept_invalid_certs(opts.all_certs);
        // Disable Referer
        builder = builder.referer(false);
        // HTTPS only
        builder = builder.https_only(opts.https_only);
        // Redirect Policy
        let policy = reqwest::redirect::Policy::limited(opts.redirects);
        builder.redirect(policy).build()
    }
    // Removes 'zeiver' from the collection of URLs
    fn clean_urls_list(opts: &mut cmd_opts::Opts) {
        if let true = opts.urls.contains(&PathBuf::from("zeiver")) {
            opts.urls.remove(0);
        }
    }
}


