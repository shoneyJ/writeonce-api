use crate::api::valid;
use crate::models::*;
use crate::services::aws::get_s3_article;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let payload = event.payload;

    let new_article: NewArticle = match serde_json::from_value(payload) {
        Ok(val) => val,
        Err(err) => {
            return Ok(json!({
                "status": "error",
                "message": format!("Invalid payload: {}", err)
            }));
        }
    };

    match Article::upsert(new_article) {
        Ok(article) => Ok(json!({
            "status": "success",
            "message": "Article saved",
            "article": article
        })),
        Err(err) => Ok(json!({
            "status": "error",
            "message": format!("Failed to save article: {}", err)
        })),
    }

    Ok(json!({ "message": format!("Article Saved") }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}
