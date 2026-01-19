use std::cell::RefCell;
use std::rc::Rc;

use crate::core::{package::Package, slide::SlideRef, Result};
use crate::core::xml;

#[derive(Debug)]
pub struct Presentation {
    package: Rc<RefCell<Package>>,
    slide_paths: Vec<String>,
}

impl Presentation {
    pub fn open(_path: &str) -> Result<Self> {
        let package = Package::open(_path)?;
        Self::from_package(package)
    }

    pub fn from_bytes(_data: &[u8]) -> Result<Self> {
        let package = Package::from_bytes(_data)?;
        Self::from_package(package)
    }

    pub fn save(&self, _path: &str) -> Result<()> {
        self.package.borrow().save(_path)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        self.package.borrow().to_bytes()
    }

    pub fn slides(&self) -> Result<Vec<SlideRef>> {
        Ok(self
            .slide_paths
            .iter()
            .enumerate()
            .map(|(index, path)| SlideRef::new(index, path, self.package.clone()))
            .collect())
    }

    pub fn replace_text(&mut self, _needle: &str, _replacement: &str) -> Result<usize> {
        let mut replaced = 0;
        let mut package = self.package.borrow_mut();
        for path in &self.slide_paths {
            if let Some(part) = package.get_part_mut(path) {
                let (updated, count) = xml::replace_text_all(&part.data, _needle, _replacement)?;
                part.data = updated;
                replaced += count;
            }
        }
        Ok(replaced)
    }
}

impl Presentation {
    fn from_package(package: Package) -> Result<Self> {
        let slide_paths = xml::slide_paths_from_package(&package)?;
        Ok(Self {
            package: Rc::new(RefCell::new(package)),
            slide_paths,
        })
    }
}
