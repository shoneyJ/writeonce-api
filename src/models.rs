use diesel::{dsl::{count_star, delete}, prelude::*};
use crate::error_handler::CustomError;
use crate::schema::articles;
use crate::db;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use diesel::insert_into;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = articles)]
pub struct NewArticle {
    pub title: String,
    pub sys_title: String,
    pub published: bool,
    pub content: Option<Value>,
    pub do_aws_sync: Option<bool>,
}

#[derive(Serialize, Deserialize,Queryable,AsChangeset, Selectable)]
#[diesel(table_name = articles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub sys_title: String,
    pub published: bool,
    pub content: Option<Value>,
    pub do_aws_sync: Option<bool>,
}

#[derive(Serialize, Deserialize,Queryable, Selectable)]
#[diesel(table_name = articles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ArticleContent {
    pub id: i32,
    pub title: String,
    pub sys_title: String,
    pub content: Option<Value>
}

#[derive(Serialize)]
pub struct CountResponse {
    count: i64,
}

#[derive(Serialize)]
pub struct RemoveResponse {
    msg: String,
}

impl Article {

    pub fn find(_id: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let article = articles::table.filter(articles::id.eq(_id)).first(&mut conn)?;
        Ok(article)
    }

    pub fn remove(_id: i32) -> Result<RemoveResponse, CustomError> {
        let mut conn = db::connection()?;
        let _ = delete(articles::dsl::articles.filter(articles::id.eq(_id))).execute(&mut conn);
        let response = RemoveResponse { msg: "removed article".to_string() };
        Ok(response)
    }

    pub fn upsert(new_article: NewArticle)  -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let article = insert_into(articles::table)
        .values(&new_article)
        .on_conflict(articles::dsl::id)
        .do_update()
        .set((
            articles::title.eq(&new_article.title),
            articles::sys_title.eq(&new_article.sys_title),
            articles::published.eq(new_article.published),
            articles::content.eq(new_article.content.clone()),
            articles::do_aws_sync.eq(new_article.do_aws_sync),
        ))
        .get_result(conn)?;

        Ok(article)

    }
    pub fn get_total_articles_count() -> Result<CountResponse, CustomError> {
        let conn = &mut db::connection()?;
        
        let total_count = articles::table
            .select(count_star()) // Use count_star to count all rows
            .first::<i64>(conn)?;

            let response = CountResponse { count: total_count };
    
        Ok(response)
    }

    pub fn get_articles_pagination(skip: i64, limit: i64) -> Result<Vec<ArticleContent>, CustomError> {

       let conn = &mut db::connection()?;
       let results = articles::table
       .select((articles::id, articles::title,articles::sys_title, articles::content))
       .limit(limit)
       .offset(skip)
       .load::<ArticleContent>(conn)?;

        Ok(results)
    }

    pub fn get_article_by_title(sys_title: String) -> Result<Self, CustomError> {

        let mut conn = db::connection()?;
        let article = articles::table.filter(articles::sys_title.eq(sys_title)).first(&mut conn)?;
        Ok(article)
     }

}
