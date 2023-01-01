// use reqwest::Response;
// use scraper::{Html, Selector};
// use std::fmt;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

// #[derive(Debug)]
// struct Trope {
//     name: String,
//     url: String,
// }

// impl fmt::Display for Trope {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         write!(f, "[{},{}]", self.name, self.url)
//     }
// }

fn main() {
    /*
    //This holds the root page
    let root_page: String = String::from("https://tvtropes.org/pmwiki/pagelist_having_pagetype_in_namespace.php?n=Main&t=trope&page=");

    // Eventually, this will be in a loop over page numbers
    let mut page: String = root_page.clone() + &1.to_string();

    // Get the page
    let response = reqwest::blocking::get(&page).unwrap().text().unwrap();

    // lol, they block robots...
    println! {"\n\n\n"}
    println!("{}", response);
    println! {"\n\n\n"}
    */

    // ugh, let's try to do the rest of it with a local copy for the moment
    // let path = Path::new("../test_data/trope_page.html");
    // let display = path.display();

    // let document: Html = match read_html_file(path) {
    //     Ok(document) => document,
    //     Err(e) => panic!("Error reading {}: {}", display, e),
    // };

    // // Create a selector for the element we want
    // // For the tropes page, every link in a table cell should get us what we want
    // // This can be done outside of the main loop, since it's the same each time and passed by reference
    // let trope_selector = Selector::parse("td>a").unwrap();

    // // Select all elements in the document
    // let trope_links = document.select(&trope_selector);

    // // For every trope, get the inner html (trope_name) and
    // let mut tropes: Vec<Trope> = Vec::new();
    // for element in trope_links {
    //     tropes.push(Trope {
    //         name: element.inner_html(),
    //         url: element.value().attr("href").unwrap().to_string(),
    //     });
    // }

    // //Test
    // //println!("{:?}", tropes);

    // // In raw form, there are two non-breaking spaces, possible line breaks and possible
    // // spaces in the middle. Let's get rid of those for simplicity. Todo: break this into
    // // seprate function.
    // for trope in tropes.iter_mut() {
    //     trope.name = String::from(&trope.name[12..]);
    //     trope.name.retain(|c| c != '\n' && c != ' ' && c != '\t');
    // }

    // // Open a file for writing
    // let output_path = "../test_data/tropes.csv";
    // let mut output = match File::create(&output_path) {
    //     Ok(output) => output,
    //     Err(why) => panic!("Couldn't write to {}: {}", output_path, why),
    // };

    // // Write all the values to the file
    // for trope in tropes.iter() {
    //     let s = format!("{}, {}\n", trope.name, trope.url);
    //     output
    //         .write_all(s.as_bytes())
    //         .expect("Unable to write string");
    // }
}

// fn read_html_file(filename: &Path) -> Result<Html, std::io::Error> {
//     // Open the file
//     let mut file = File::open(&filename)?;

//     // Read the file
//     let mut html = String::new();
//     file.read_to_string(&mut html)?;

//     // Convert to Html
//     let document = Html::parse_document(&html);

//     Ok(document)
// }
