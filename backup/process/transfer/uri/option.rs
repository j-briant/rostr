/// RDatasetoptions are GDAL Datasetoptions
#[derive(Debug, Deserialize, PartialEq)]
struct RDatasetOption {
    pub open_flags: Vec<String>,
    pub allowed_drivers: Vec<String>,
    pub open_options: Vec<String>,
    pub sibling_files: Vec<String>,
}