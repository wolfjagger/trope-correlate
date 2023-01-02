use std::fmt;

use derive_more::Display;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{KNOWN_MEDIA_NAMESPACES, KNOWN_TROPE_NAMESPACES};


#[derive(Debug, Display, Clone, Copy, Deserialize, Serialize)]
pub enum NamedLinkType {
  Trope,
  Media,
  Other,
}


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
  link_type: NamedLinkType,
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

  pub fn link_type(&self) -> NamedLinkType {
    self.link_type
  }

  fn calc_link_type(url: &str) -> NamedLinkType {

    static NAMESPACE_LINK_RE: Lazy<Regex> = Lazy::new(||
      Regex::new(
        "/pmwiki/pmwiki.php/([^/]*)/(.*)"
      ).expect("Error creating namespace regex")
    );

    match NAMESPACE_LINK_RE.captures(url).and_then(|cap| {
      let ns = cap.get(1).map(|m| m.as_str());
      let link_name = cap.get(2).map(|m| m.as_str());
      ns.and_then(|ns| link_name.map(|ln| (ns, ln)))
    }) {
      Some((namespace, _link_name)) => {
        if KNOWN_TROPE_NAMESPACES.iter().any(|s| s == &namespace) {
          NamedLinkType::Trope
        } else if KNOWN_MEDIA_NAMESPACES.iter().any(|s| s == &namespace) {
          NamedLinkType::Media
        } else {
          NamedLinkType::Other
        }
      },
      None => NamedLinkType::Other
    }

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
    write!(f, "[{},{}, {}]", self.name, self.url, self.link_type)
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
