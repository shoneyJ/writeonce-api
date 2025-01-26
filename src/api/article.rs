use actix_web::{get,post,delete,web, HttpResponse,HttpRequest, Responder};
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

#[delete("/article/{id}")]
pub async fn remove_article_by_id( req: HttpRequest,id: web::Path<i32>) -> impl Responder {
    if let Err(response) = valid::validate_token(&req).await {
        return response; // Return Unauthorized response
    }

    match Article::remove(id.into_inner()) {
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

#[get("/article/title/{sys_title}")]
pub async fn get_article_by_title( req: HttpRequest,sys_title: web::Path<String>) -> impl Responder {
    if let Err(response) = valid::validate_token(&req).await {
        return response; // Return Unauthorized response
    }

    match Article::get_article_by_title(sys_title.into_inner()) {
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

#[get("/articles/count")]
pub async fn get_articles_total_count (req: HttpRequest) -> impl Responder {

    if let Err(response) = valid::validate_token(&req).await {
        return response; // Return Unauthorized response
    }

    match Article::get_total_articles_count() {
        Ok(result) => {
            // Return the article in the response
            HttpResponse::Ok().json(result)
        },
        Err(_) => {
            // Handle the case when the article is not found
            HttpResponse::NotFound().body("Article not found")
        }
    }


}

#[get("/articles/{skip}/{limit}")]
pub async fn get_articles_pagination(req: HttpRequest,path: web::Path<(i64,i64)>)-> impl Responder {

    if let Err(response) = valid::validate_token(&req).await {
        return response; 
    }

    let (skip, limit) = path.into_inner();

    match Article::get_articles_pagination(skip, limit) {
        Ok(articles) => {
            // Return the article in the response
            HttpResponse::Ok().json(articles)
        },
        Err(_) => {
            // Handle the case when the article is not found
            HttpResponse::NotFound().body("Articles not found")
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