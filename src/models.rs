use diesel::{dsl::count_star, prelude::*};
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

#[derive(Serialize, Deserialize,Queryable, Selectable)]
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

impl Article {

    pub fn find(_id: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let article = articles::table.filter(articles::id.eq(_id)).first(&mut conn)?;
        Ok(article)
    }

    pub fn upsert(new_article: NewArticle)  -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let article = insert_into(articles::table)
        .values(&new_article)
        .on_conflict(articles::id)
        .do_update()
        .set(&new_article)
        .get_result(conn)?;

        Ok(article)

    }
    pub fn get_total_articles_count() -> Result<i64, CustomError> {
        let conn = &mut db::connection()?;
        
        let total_count = articles::table
            .select(count_star()) // Use count_star to count all rows
            .first::<i64>(conn)?;
    
        Ok(total_count)
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

}
