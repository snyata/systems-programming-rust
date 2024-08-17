/// # Llina Rust Community Machine Learning Project
/// ***Implementation of the Logistic Regression Classifier***
/// 
/// ## Objective:
/// #### Input: 
/// - JSON output from the ml_endpoint_scanner
///     - identified ml_endpoints for further analysis
/// - Clustering as an experimental middle step for identifying like-endpoints
/// - Naive Bayes
/// - Decision Trees
///
/// #### Notes:
/// - Can label the output based on rule based criterium to use a Supervized Approach
/// 
/// 

use linfa::composing::MultiClassModel;
use linfa::prelude::*;
use linfa::metrics::ToConfusionMatrix;
use linfa::traits::{Fit, Predict, Transformer};
use linfa_bayes::GaussianNb;
use linfa_preprocessing::linear_scaling::LinearScaler;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};



println!(data_post);

pub async fn pre_process(data: DataInput) {

}







