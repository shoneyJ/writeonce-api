pub mod article;
pub mod valid;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(article::get_article_by_id);
    cfg.service(article::upsert);
}