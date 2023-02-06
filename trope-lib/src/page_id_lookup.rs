use std::path::Path;

use bimap::BiMap;

use crate::PageId;


pub struct PageIdLookup {
  bimap: BiMap<u32, String>
}

impl PageIdLookup {

  pub fn new<I>(page_ids: I) -> Self
  where I: IntoIterator<Item=PageId> {
    Self{
      bimap: BiMap::from_iter(page_ids.into_iter().map(|pi| pi.into()))
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

}
