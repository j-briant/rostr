pub mod driver;
pub mod option;

use gdal::{errors::GdalError, Dataset, DatasetOptions, GdalOpenFlags};
use serde::Deserialize;

trait Connection {
    fn touch(&self) -> bool;
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SourceURI<'a> {
    path: String,
    options: DatasetOptions<'a>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DestinationURI {
    path: String,
}

impl Connection for SourceURI {
    fn touch(&self) -> bool {
        match Dataset::open_ex(self.path, self.options) {
            Ok(_) => true,
            _ => false,
        }
    }
}

// Error type for URI.
#[derive(Debug)]
enum UriError {
    ConnectionError,
}

impl SourceURI {
    pub fn from_str(p: &str) -> Result<SourceURI, UriError> {
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
    fn uri_test_connection() {
        let path = "PG:dbname=osm_suisse tables=planet_osm_line";
        let uri = SourceURI::from_str(&path).unwrap();
        assert_eq!(uri.path, path);
    }

    #[test]
    fn uri_test_connection_error() {
        let path = "PG:dbname=osm_susse tables=planet_osm_line";
        let error = SourceURI::from_str(&path).map_err(|e| e.kind());
        assert_eq!(error, UriError::ConnectionError);
    }
}
