pub mod article;
pub mod valid;
pub mod spec;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(article::get_article_by_id);
    cfg.service(article::upsert);
    cfg.service(article::get_articles_pagination);
    cfg.service(article::get_articles_total_count);
    cfg.service(article::get_article_by_title);
    cfg.service(article::remove_article_by_id);
    cfg.service(spec::openapi_spec);
    cfg.service(spec::swagger_ui);
}