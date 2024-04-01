pub mod uri;

use serde::Deserialize;

/// Operations are the valid GDAL/OGR operation possible on a dataset (update, overwrite etc.)
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Operation {
    Overwrite,
    Update,
    Upsert,
}

#[derive(Debug)]
pub struct Transfer {
    #[serde(default = "get_id")]
    id: i32,
    operation: Operation,
    src: uri::SourceURI,
    dst: uri::DestinationURI,
}

fn get_id() -> i32 {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_from_str() {
        let input = r#"
            {
                "operation": "update",
                "src_dataset": "PG:dbname=osm_suisse tables=planet_osm_line",
                "dst_dataset": "localhost::dst_dataset"
            }"#;
        let uri = uri::SourceURI::from_str("PG:dbname=osm_suisse tables=planet_osm_line").unwrap();
        assert_eq!(
            Transfer::from_str(&input).unwrap(),
            Transfer {
                id: 0,
                operation: Operation::Update,
                src: uri,
                dst: String::from("localhost:dst_dataset"),
            }
        );
    }

    #[test]
    #[should_panic]
    fn err_on_missing_field() {
        // Missing src_dataset
        let input = "
            {
                \"operation\": \"update\",
                \"dst_dataset\": \"localhost::dst_dataset\"
            }";
            Transfer::from_str(&input).unwrap();
    }
}
