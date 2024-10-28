use actix_web::{get,post,web, HttpResponse,HttpRequest, Responder};
use crate::models::*;
use crate::api::valid;
use crate::services::aws::get_s3_article;

#[get("/article/{id}")]
pub async fn get_article_by_id( req: HttpRequest,id: web::Path<i32>) -> impl Responder {
    if let Err(response) = valid::validate_token(&req).await {
        return response; // Return Unauthorized response
    }

    match Article::find(id.into_inner()) {
        Ok(article) => {
            // Return the article in the response
            HttpResponse::Ok().json(article)
        },
        Err(_) => {
            // Handle the case when the article is not found
            HttpResponse::NotFound().body("Article not found")
        }
    }

}

#[post("/article/")]
pub async fn upsert( req: HttpRequest, body: web::Json<NewArticle>) -> impl Responder {
    if let Err(response) = valid::validate_token(&req).await {
        return response; // Return Unauthorized response
    }
    let mut new_article= body.into_inner();
    
    if new_article.do_aws_sync.unwrap_or(false){

        match get_s3_article(new_article.sys_title.clone()).await {
            Ok(content) =>{

                new_article.content = Some(content);

            }
            Err(_) => {
                
            }
        };

    }


    match Article::upsert(new_article) {
        Ok(article) => {
            // Return the article in the response
            HttpResponse::Ok().json(article)
        },
        Err(_) => {
            // Handle the case when the article is not found
            HttpResponse::NotFound().body("Failed to Insert")
        }
    }

}