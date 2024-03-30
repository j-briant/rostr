pub mod driver;

use gdal::{errors::GdalError, Dataset, DatasetOptions, DriverManager, GdalOpenFlags};

struct SourceURI {
    path: String,
}

// Error type for URI.
#[derive(Debug)]
enum UriError {
    ConnectionError,
}

impl SourceURI {
    fn from_str(p: &str) -> Result<SourceURI, UriError> {
        let opt = DatasetOptions {
            open_flags: GdalOpenFlags::GDAL_OF_ALL,
            allowed_drivers: Some(&[]),
            ..Default::default()
        };
        match Dataset::open_ex(p, opt) {
            Ok(_) => Ok(SourceURI {
                path: p.to_string(),
            }),
            Err(e) => {
                println!("Error: {e}");
                Err(UriError::ConnectionError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        let path = "PG:dbname=osm_suisse tables=planet_osm_line";
        let uri = SourceURI::from_str(&path).unwrap();
        assert_eq!(uri.path, path);
    }
}
