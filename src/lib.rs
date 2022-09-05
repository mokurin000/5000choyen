use std::path::PathBuf;

use pyo3::prelude::*;
mod choyen_5000;

pub fn generate_5000choyen(
    top: &str,
    bottom: &str,
    file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    Python::with_gil(|py| {
        let generator = PyModule::from_code(py, choyen_5000::CODE, "", "").unwrap();

        generator
            .call_method("genImage_to", (top, bottom, file), None)
            .unwrap();
    });

    Ok(())
}
