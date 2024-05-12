use super::{model, DatabasePool};
use crate::data;
use sqlx::{query, query_as};

type Result<T> = std::result::Result<T, data::DataError>;

pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model: model::GetClip = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(query_as!(
        model::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode
    )
    .fetch_one(pool)
    .await?)
}

pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model: model::NewClip = model.into();
    let _ = query!(
        "INSERT INTO clips (clip_id, shortcode, content, title, posted, expires, password, hits) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted, 
        model.expires, 
        model.password, 
        0
    ).execute(pool).await?;

    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool
) -> Result<model::Clip> {
    let model: model::UpdateClip = model.into();    

    let _ = query!(
        "UPDATE clips SET content = ?, title = ?, expires = ?, password = ? WHERE shortcode = ?",
        model.content,
        model.title,
        model.expires,
        model.password,
        model.shortcode,
    ).execute(pool).await?;

    get_clip(model.shortcode, pool).await
}