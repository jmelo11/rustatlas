use quantsupport::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
struct JsonCurveSpecs {
    curve_specs: Vec<CurveConfiguration>,
}

pub fn load_quotes(path: &PathBuf) -> Result<QuoteStore> {
    let file =
        File::open(path).map_err(|e| QSError::NotFoundErr(format!("{}: {e}", path.display())))?;
    let reader = BufReader::new(file);
    let records: QuoteStoreRecords =
        serde_json::from_reader(reader).map_err(|e| QSError::InvalidValueErr(e.to_string()))?;
    QuoteStore::try_from(records)
}

pub fn load_curve_specs(path: &PathBuf) -> Result<Vec<CurveConfiguration>> {
    let file =
        File::open(path).map_err(|e| QSError::NotFoundErr(format!("{}: {e}", path.display())))?;
    let reader = BufReader::new(file);
    let json: JsonCurveSpecs =
        serde_json::from_reader(reader).map_err(|e| QSError::InvalidValueErr(e.to_string()))?;
    Ok(json.curve_specs)
}
