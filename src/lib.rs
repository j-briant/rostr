use gdal::{spatial_ref::SpatialRef, Dataset, DriverManager, GeoTransform};
use std::panic;

struct RostrDataset{
    data: Dataset,
    metadata: RostrMetadata
}

#[derive(Debug, PartialEq)]
pub struct RostrMetadata {
    geotransform: GeoTransform,
    projection: String,
    spatial_ref: SpatialRef,
    size: (usize, usize)
}

#[derive(Debug)]
pub enum MetadataError {
    DatasetError(String)
} 


impl RostrMetadata {
    fn from(dst: &Dataset) -> Result<Self, MetadataError> {
        let result = panic::catch_unwind(|| {
            dst.geo_transform();
            dst.spatial_ref()
        });
        if result.is_ok() {
            if let (Ok(gt), Ok(sr)) = (dst.geo_transform(), dst.spatial_ref()) {
                Ok(RostrMetadata{
                    geotransform: gt,
                    projection: dst.projection(),
                    spatial_ref: sr,
                    size: dst.raster_size()
                })
            } else {
                Err(MetadataError::DatasetError("Error while getting metadata from the dataset.".into()))
            }
        } else {
            Err(MetadataError::DatasetError("Error while getting metadata from the dataset.".into()))
        }
    }

    fn geotransform(&self) -> &[f64; 6] {
        &self.geotransform
    }

    fn projection(&self) -> &str {
        &self.projection
    }

    fn spatial_ref(&self) -> &SpatialRef {
        &self.spatial_ref
    }

    fn size(&self) -> &(usize, usize) {
        &self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_from() {
        let drv = DriverManager::get_driver_by_name("MEM").unwrap();
        let ds = drv.create("", 5, 5, 1).unwrap();
        assert_eq!(RostrMetadata::from(&ds).expect("Error while loading metadata"), RostrMetadata{
            geotransform: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            projection: "".into(),
            spatial_ref: SpatialRef::new().unwrap(),
            size: (5, 5)
        });
    }
}