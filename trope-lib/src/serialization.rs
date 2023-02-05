use std::{fmt, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{Namespace, EntityType};


#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeNamedLink {
  pub name: String,
  pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(from = "SerdeNamedLink", into = "SerdeNamedLink")]
pub struct NamedLink {
  pub name: String,
  pub url: String,
  link_type: EntityType,
  url_page_name: String,
}

impl NamedLink {

  pub fn new(name: String, mut url: String) -> Self {
    // If an internal link, prepend the site
    if url.starts_with("/pmwiki") {
      url.insert_str(0, "https://tvtropes.org");
    }
    let (link_type, url_page_name) = Self::calc_url_byproducts(&url);
    NamedLink{
      name, url, link_type, url_page_name
    }
  }

  pub fn link_type(&self) -> EntityType {
    self.link_type
  }

  pub fn url_page_name(&self) -> &str {
    &self.url_page_name
  }

  fn calc_url_byproducts(url: &str) -> (EntityType, String) {

    static NAMESPACE_LINK_RE: Lazy<Regex> = Lazy::new(||
      Regex::new(
        "/pmwiki/pmwiki.php/([^/]*)/(.*)"
      ).expect("Error creating url regex")
    );

    NAMESPACE_LINK_RE.captures(url).and_then(|cap| {

      let ns = cap.get(1).map(|m| m.as_str()).and_then(|ns| Namespace::from_str(ns).ok());
      let link_name = cap.get(2).map(|m| m.as_str());
      ns.zip(link_name).map(|(ns, ln)| (ns.entity_type(), ln.to_owned()))

    }).expect(&format!("Could not parse url {}", url))

  }

}

impl From<SerdeNamedLink> for NamedLink {
  fn from(s: SerdeNamedLink) -> Self {
    Self::new(s.name, s.url)
  }
}
impl From<NamedLink> for SerdeNamedLink {
  fn from(s: NamedLink) -> Self {
    Self {
      name: s.name,
      url: s.url,
    }
  }
}

impl fmt::Display for NamedLink {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}, {}, {}]", self.name, self.url, self.link_type)
  }
}



#[derive(Debug, Deserialize, Serialize)]
pub struct TropeGeneralJson {
  pub title: String,
  pub subpages: Vec<NamedLink>,
}

impl Default for TropeGeneralJson {
  fn default() -> Self {
    Self {
      title: String::default(),
      subpages: Vec::default(),
    }
  }
}


#[derive(Serialize, Deserialize)]
pub struct PageId {
  pub id: u32,
  pub page: String,
}
