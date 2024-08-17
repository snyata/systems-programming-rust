/// Preparing a dataset from a Vector of observations into a HashMap
/// 

use linfa::prelude::*;
use linfa_preprocessing::linear_scaling::LinearScaler;
use tokio::prelude::*;
use linfa::metrics::ToConfusionMatrix;
use linfa::traits::{Fit, Predict, Transformer};
use linfa_bayes::GaussianNb;
use linfa_logistic::error::Result;
use linfa_logistic::LogisticRegression;

struct DataInput {
    id: i16
    url: String
    src: String
    label: bool
}

static ENVIRONMENT = "DEV"
if ENVIRONMENT == "DEV" {
    let (train, valid) = linfa_datasets::winequality()
        .map_targets(|x| if *x > 6 { 1 } else { 0 })
        .split_with_ratio(0.8)
} else {
    let dataset:
}

let data_example = serde_json::json({"id": 1, "url": "https://www.google.com", "src": "https://www.google.com", "label": true},
                        {"id": 2, "url": "https://www.google.com", "src": "https://www.google.com", "type": "ml_endpoint", "label": true});

pub fn preprocess_data(data) {
    // Initialize LinearScaler for Normalization
    let scaler = LinearScaler::standard().fit(&train).unwrap();

    // Scale Training and Validation Sets
    let train = scaler.transform(train);
    let validate = scaler.transform(validate);

    //Apply a Naive Bayes model
   let model = GaussianNb::params().fit(&train).unwrap();

   // Compute Accuracies
   let train_acc = model
       .predict(&train)
       .confusion_matrix(&train)
       .unwrap()
       .accuracy();
    
    let cm = model.predict(&valid).confusion_matrix(&valid).unwrap();
    let valid_acc = cm.accuracy();
    println!(
        "Train Accuracy: {:.2}\nValidation Accuracy: {:.2}",
        train_acc, valid_acc
    )
    println!( "{?}", cm );

}

enum Models {
    NaiveBayes,
    DecisionTree,
    LogisticRegression,
    RandomForest
}

async fn define_model(models: Models, model: Models) -> Models<Ok, Error> {
    match models {
        NaiveBayes => model == GaussianNb;
        DecisionTree => model == DecisionTree;
        LogisticRegression => model == LogisticRegression;
        RandomForest => model == RandomForest;
        _ => println!("Model not found")
    }

    if model == GaussianNb {
        params = GaussianNb::params().to_string()
    } else if model == DecisionTree {
        params = DecisionTree::params().to_string()
    } else if model == LogisticRegression {
        params = LogisticRegression::params().to_string()
    } else if model == RandomForest {
        params = RandomForest::params().to_string()
    }
    Ok((model, params))
}

pub async fn train_model(model: Models, params: &str) {

}

async fn

//Apply a Naive Bayes model
let model = GaussianNb::params().fit(&train).unwrap();
