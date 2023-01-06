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
}

impl NamedLink {

  pub fn new(name: String, mut url: String) -> Self {
    let link_type = Self::calc_link_type(&url);
    // If an internal link, prepend the site
    if url.starts_with("/pmwiki") {
      url.insert_str(0, "https://tvtropes.org");
    }
    NamedLink{
      name, url, link_type
    }
  }

  pub fn link_type(&self) -> EntityType {
    self.link_type
  }

  fn calc_link_type(url: &str) -> EntityType {

    static NAMESPACE_LINK_RE: Lazy<Regex> = Lazy::new(||
      Regex::new(
        "/pmwiki/pmwiki.php/([^/]*)/(.*)"
      ).expect("Error creating namespace regex")
    );

    NAMESPACE_LINK_RE.captures(url).and_then(|cap| {

      let ns = cap.get(1).map(|m| m.as_str()).and_then(|ns| Namespace::from_str(ns).ok());
      let link_name = cap.get(2).map(|m| m.as_str());
      ns.zip(link_name).map(|(ns, _ln)| ns.entity_type())

    }).unwrap_or(EntityType::Other)

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
