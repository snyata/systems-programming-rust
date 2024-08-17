/// Take Batches of Data and send to LLM
/// 
/// 

mod data;

use py03::prelude::*;
use py03::types::PyModule;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Sender};
use tokio::task;
use serde_json;
use data::read_json;

fn process_batch_with_python(batch: Vec<InputData>) -> PyResult<Vec<OutputData>> {
    let input_json = serde_json::to_string(&batch).unwrap();

    Python::with_gil(|py| {

        let llama_index = PyModule::from_code(py, include_str!("../scripts/py_llama.py"), "py_llama.py", "py_llama")?;
        let process_batch = llama_index.getattr("process_batch")?;
        let output_json: String = process_batch.call1((input_json,))?.extract()?;

        // Deserialize the JSON result back into a vector of OutputDat
        let output_data: Vec<OutputData> = serde_json::from_str(&output_json).unwrap();

        Ok(output_data)
    })
}