use gdal::errors::GdalError;
use gdal::DriverManager;

#[derive(Debug)]
pub struct DriverList {
    count: usize,
    list: Vec<String>,
}

impl DriverList {
    pub fn get() -> Result<DriverList, GdalError> {
        DriverManager::register_all();
        let count = DriverManager::count();
        let mut list: Vec<String> = vec![];
        for i in 0..count {
            list.push(DriverManager::get_driver(i)?.short_name())
        }
        Ok(DriverList {
            count: count,
            list: list,
        })
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
}
