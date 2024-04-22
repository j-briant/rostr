use gdal::{DriverManager, DatasetOptions, GdalOpenFlags};
use gdal::errors::GdalError;

const DRIVER_LIST: Vec<&str> = drivers().unwrap();

pub fn drivers() -> Result<Vec<&'static str>, GdalError> {
    DriverManager::register_all();
    let count = DriverManager::count();
    let mut list: Vec<&'static str> = vec![];
    for i in 0..count {
        list.push(&DriverManager::get_driver(i)?.short_name())
    }
    Ok(list)
}

pub struct RostrOpenOption(Vec<String>);

impl TryFrom for RostrOpenOption {
    fn try_from(value: T) -> Result<Self, Self::Error> {
        
    }
}

pub struct RostrDatasetOption {
    open_flags: GdalOpenFlags::GDAL_OF_ALL, 
    allowed_drivers: Some(Vec<String>), 
    open_options: Some(&s), 
    sibling_files: None 
}
    
    
    
    //<'a>(gdal::DatasetOptions<'a>);

impl TryFrom<Vec<&str>> for RostrDatasetOption<'_> {
    type Error = gdal::errors::GdalError;

    fn try_from(s: Vec<&str>) -> Result<Self, Self::Error> {
        Ok(RostrDatasetOption(gdal::DatasetOptions { 
            open_flags: GdalOpenFlags::GDAL_OF_ALL, 
            allowed_drivers: Some(&DRIVER_LIST[..]), 
            open_options: Some(&s), 
            sibling_files: None }))
    }
}

pub struct RostrSource(gdal::Dataset);

impl TryFrom<Source> for RostrSource {
    type Error = gdal::errors::GdalError;

    fn try_from(s: Source) -> Result<Self, Self::Error> {
        let my_opt: RostrDatasetOption = s.option.try_into()?;
        let rs = RostrSource(gdal::Dataset::open_ex(s.path, s.option)?);
        Ok(rs)
    }
}
