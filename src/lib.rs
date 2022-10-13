use std::path::PathBuf;

use pyo3::prelude::*;
mod choyen_5000;

pub fn generate_5000choyen(
    top: &str,
    bottom: &str,
    file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    Python::with_gil(|py| {
        let generator = PyModule::from_code(py, choyen_5000::CODE, "", "").unwrap();

        generator
            .call_method(
                "genImage_to",
                (if !top.is_empty() { top } else { "5000兆円" }, 
                if !bottom.is_empty() { bottom } else { "欲しい!" }, file),
                None,
            )
            .unwrap();
    });

    Ok(())
}
