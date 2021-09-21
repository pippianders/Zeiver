# Zeiver
**Zeiver** is designed to *scrape* and *download content* recursively from ODs _(open directories)_.
It also provides a means of *retrieving links* to the content and *scouting*.

__*Zeiver does not download the entire OD itself, only the files.__

For ease of use, check out the [Zeiver configurator](https://zimtools.xyz/zeiver).

## Table of Contents
* [Features](https://github.com/ZimCodes/Zeiver#features)
   * [Normal Workflow](https://github.com/ZimCodes/Zeiver#normal-workflow)
   * [More](https://github.com/ZimCodes/Zeiver#more)
* [Supported ODs](https://github.com/ZimCodes/Zeiver#supported-ods)
* [Installation](https://github.com/ZimCodes/Zeiver#installation)
* [Sample](https://github.com/ZimCodes/Zeiver#sample)
* [Commands](https://github.com/ZimCodes/Zeiver#commands)
    * [Positional](https://github.com/ZimCodes/Zeiver#positional) 
    * [Options](https://github.com/ZimCodes/Zeiver#options)
        * [General](https://github.com/ZimCodes/Zeiver#general)
        * [Download](https://github.com/ZimCodes/Zeiver#download)
        * [Recorder](https://github.com/ZimCodes/Zeiver#recorder)
        * [File/Directory](https://github.com/ZimCodes/Zeiver#filedirectory)
        * [HTTP](https://github.com/ZimCodes/Zeiver#http)
* [Tips/Tricks](https://github.com/ZimCodes/Zeiver#tips-and-tricks)
* [License](https://github.com/ZimCodes/Zeiver#license)

## Features
Zeiver currently has 4 major modules:
* HTTP
  * Grabs content from the internet. *(Webpage,files,etc)*
* Scraper
    * Recursively grabs all links from an OD.
* Downloader
    * Downloads content retrieved from Scraper (_or from a file_)
* Recorder
    * Saves a record of all files that were found in the OD
    * Records are saved to a file called *URL_Records.txt*. Name can be 
      changed using `--output-record`
    * Creates stat files *(statistical data about what was retrieved)*

***All components can be used independently.*** With exception to *HTTP module*.

### Normal Workflow
The **HTTP** module repeatedly grabs a webpage for the Scraper to parse *(based on parameters)*.
The **Scraper** takes the webpage and recursively grabs the links from them.
Afterwards, the links are either sent to the
**Recorder** (_Disabled by default_), specified with:
* `--record-only`
* `--record`
 
*AND/OR*
 
**Downloader** (_Enabled by default_). The **Downloader** uses the **HTTP** module to download
the files' data from the internet. the **Downloader** then writes the data to a newly created files.

### More
1. Uses asynchronous runtime

2. Random & fixed delays of HTTP requests

3. Ability to customize how files retrieved or not

## Supported ODs
List of currently supported ODs:
* OLAINDEX
* Directory Lister
* AutoIndex PHP
* Abyss Web Server X1 & X2
* LiteSpeed Web Server
* Apache
* NGINX
* Evoluted Directory Listing Script

**May also work with other ODs*.

## Installation
1. Install Rust. 
    * If Rust is not installed, please follow the instructions [here](https://www.rust-lang.org/tools/install)
    
2.  Once Rust is installed, open a CLI & type `cargo install --branch main --git https://github.com/ZimCodes/Zeiver`
    
    * _This will install Zeiver from Github_
    
3. And that's it! To use Zeiver, start each command with `zeiver`.

## Sample
The following code downloads files from _example.com/xms/imgs_, saves them in a local directory called _Cool_Content_,
& sends a request with the ACCEPT-LANGUAGE header.

`zeiver -h "accept-language$fr-CH, en;q=0.8, de;q=0.7" -o "./Cool_Content" example.com/xms/imgs`

## Commands 
### Positional
__URL*s*...__

Link(*s*) to the OD(*s*) you would like to download content from. 
*_This is not needed if you are using the `-i, --input-file` option_

---
### Options
#### General
**_-h, --help_**

Prints help information.

***-V, --version***

Prints version information

***-v, --verbose***

Enable verbose output

***--test***

Run a scrape test without downloading or recording. **Can be used with `--verbose`
to scout an OD's available content *like a spider***.

---
#### Download

***-d, --depth***

Specify the maximum depth for recursive scraping. Can also be used to traverse subpages *(ODs with **previous** & **next** buttons)*.
_Default: `20`_. **Depth of`1` is current directory.**

***-A, --accept***

Files to accept for scraping

Using Regex, specify which files to accept for scraping. Only the files that matches the regex will be
acceptable for download. _*This option takes precedence over `--reject, -r`_.

Ex: `zeiver -A "(mov|mp3|lunchbox_pic1\.jpg|(pic_of_me.gif))"`

***-R, --reject***

Files to reject for scraping

Using Regex, specify which files to reject for scraping. Only the files that match the regex will be
rejected for download. *_`--accept, -a` takes precedence over this option_.

Ex: `zeiver -R "(jpg|png|3gp|(pic_of_me.gif))"`

---
#### Recorder

***--record***

Activates the Recorder

Enables the Recorder which saves the scraped links to a file.
*_Option cannot be used with `--record-only`_.

***--record-only***

Save the links only

After scraping, instead of downloading the files, save the links to them. *_The downloader will be disabled when
this option is active. Enables Recorder instead._

***--output-record***

Changes the name of the record file. This file is where the recorder will store the links. *Default: `URL_Records.txt`*

***--no-stats***

Prevents Recorder from creating `_stat_` files.

The Recorder will no longer create `_stat_` files when saving scraped links to a file. *Default: `false`*
Ex: `stat_URL_Record.txt`

---
#### File/Directory

***-i, --input-file***

Read URLs from a file to be sent to the Scraper. *Each line represents a URL to an OD.

Ex: `zeiver -i "./dir/urls.txt"`

***--input-record***

Read URLs from a file containing file paths and create a stats file based on the results.
*Each line represents a URL to a file. **Activates Recorder**. Valid with `--verbose`,
`--output`, `--output-record`

***-o, --output***

Save file location.

The local file path to save files. **Files saved by the *Recorder* are also stored here.**
_Default: `./`_

Ex: `zeiver -o "./downloads/images/dir"`

***-c,--cuts***

Ignores a specified number of remote directories from being created.
*_Only available when downloading. Default: `0`_

Ex: URL: `example.org/pub/xempcs/other/pics`

Original Save Location: `./pub/xempcs/other/pics`

`zeiver --cuts 2 www.example.org/pub/xempcs/other/pics` 

New Save Location: `./other/pics`

***--no-dirs***

Do not create a hierarchy of directories structured the same as the URL the file came from. 
All files will be saved to the current output directory instead. 

*_Only available when downloading._

---
#### HTTP
***-H, --headers***

Sets the default headers to use for every request. *_Must use the __'header$value'__ format. Each header must also be
**separated by a comma**._ 

Ex: `zeiver -H content-length$128,"accept$ text/html, application/xhtml+xml, image/webp"`

***-U***

The User Agent header to use. _Default: `Zeiver/VERSION`_

***-t, --tries***

The amount of times to retry a failed connection/request. _Default: `20`_
 
***-w, --wait***

Wait a specified number of seconds between each scraping & download requests.

***--retry-wait***

The wait time between each failed request. _Default: `10`_

***--random-wait***

Wait a random amount of seconds between each request.

The time between requests will vary between 0.5 * `--wait,-w` (_inclusive_) to 1.5 * `--wait,-w` (_exclusive_)

***-T, --timeout***

Adds a request timeout for a specified number of seconds.

***-r, --redirects***

Maximum redirects to follow. _Default: `10`_

***--proxy***

The proxy to use.

Ex: `zeiver --proxy "socks5://192.168.1.1:9000"`

***--proxy-auth***

The basic authentication needed to use the proxy. *_Must use the __'username:password'__ format._

## Tips and Tricks
### URL is too long
Having trouble entering a long URL in the terminal? Place them inside an input file and use `--input-file` instead.


## License 
Zeiver is licensed under the MIT and Apache 2.0 Licenses.

See the [MIT](https://github.com/ZimCodes/Zeiver/blob/main/LICENSE-MIT) and [Apache-2.0](https://github.com/ZimCodes/Zeiver/blob/main/LICENSE-APACHE_2.0) for more details.  

