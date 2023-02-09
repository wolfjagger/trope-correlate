use std::{mem::size_of_val, path::Path};

use bimap::BiMap;

use crate::{NamedLink, PageId};


pub struct PageIdLookup {
  bimap: BiMap<u32, String>
}

impl PageIdLookup {

  pub fn new<I>(pageids: I) -> Self
  where I: IntoIterator<Item=PageId> {
    Self{
      bimap: BiMap::from_iter(pageids.into_iter().map(|pi| pi.into()))
    }
  }

  pub fn from_path(p: &Path) -> Result<Self, csv::Error> {
    let pageids: Result<Vec<PageId>, _> = csv::Reader::from_path(p)?.into_deserialize().collect();
    Ok(Self::new(pageids?.into_iter()))
  }

  pub fn contains_page(&self, page: &str) -> bool {
    self.bimap.contains_right(page)
  }
  pub fn contains_id(&self, id: &u32) -> bool {
    self.bimap.contains_left(id)
  }

  pub fn get_page(&self, id: &u32) -> Option<&String> {
    self.bimap.get_by_left(id)
  }
  pub fn get_id(&self, page: &str) -> Option<&u32> {
    self.bimap.get_by_right(page)
  }

  pub fn pageid_from_page(&self, page: &str) -> Option<PageId> {
    self.get_id(page).map(|id| PageId{ id: id.clone(), page: page.to_string() })
  }
  pub fn pageid_from_id(&self, id: &u32) -> Option<PageId> {
    self.get_page(id).map(|page| PageId{ id: id.clone(), page: page.to_string() })
  }

  pub fn len(&self) -> usize {
    self.bimap.len()
  }

  pub fn byte_size(&self) -> usize {
    self.bimap.left_values().map(|s| size_of_val(&*s)).sum::<usize>() +
      self.bimap.right_values().map(|s| size_of_val(&*s)).sum::<usize>()
  }

  pub fn pageids_from_path(&self, p: &Path)
  -> Result<(Vec<PageId>, Vec<String>), csv::Error> {

    let mentioned_pages = path_to_page_names(&p)?;

    let (found_pages, missing_pages): (Vec<_>, Vec<_>) = mentioned_pages.into_iter().partition(
      |name| self.contains_page(&name)
    );
    let ment_page_pageids: Vec<_> = found_pages.into_iter().map(
      |name| self.pageid_from_page(&name).unwrap()
    ).collect();

    log::trace!("Found pageids:");
    for t_id in &ment_page_pageids {
      log::trace!("{}", t_id);
    }

    log::trace!("Missing pages:");
    for missing_page in &missing_pages {
      log::trace!("{}", missing_page);
    }

    Ok((ment_page_pageids, missing_pages))

  }

}


fn path_to_page_names(p: &Path) -> Result<Vec<String>, csv::Error> {
  // Note: url is the source of truth in these mentions; short name is different page-to-page
  let mentions = csv::Reader::from_path(p)?.into_deserialize::<NamedLink>();
  mentions.map(|m_result| m_result.map(|m| m.url_page_name().to_string())).collect()
}
