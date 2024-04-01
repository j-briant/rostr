use std::fmt::Display;

use gdal::errors::GdalError;
use gdal::DriverManager;

/// GDAL has a certain number of drivers accessible depending on how it's installed. DriverList is just here to store the number of drivers and their list.
#[derive(Debug)]
pub struct DriverList {
    count: usize,
    list: Vec<String>,
}

impl DriverList {
    /// Get the driver list from the environment.
    ///
    /// # Example
    /// ```
    /// use rostr::uri::driver::DriverList
    ///
    /// let driver_list = DriverList::get().unwrap();
    /// ```
    pub fn get() -> Result<DriverList, GdalError> {
        DriverManager::register_all();
        let count = DriverManager::count();
        let mut list: Vec<String> = vec![];
        for i in 0..count {
            list.push(DriverManager::get_driver(i)?.short_name())
        }
        Ok(DriverList { count, list })
    }
}

impl Display for DriverList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DriverList({}, {:?})", self.count, self.list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_list_not_empty() {
        let drivers = DriverList::get().unwrap();
        assert!(drivers.count > 0)
    }

    #[test]
    fn test_display_trait() {
        let drivers = DriverList::get().unwrap();
        println!("{drivers}");
    }
}
