use super::DbId;
use crate::{domain, service::ask, ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}

impl TryFrom<Clip> for domain::Clip {
    type Error = ClipError;
    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field::*;
        use std::str::FromStr;
        Ok(Self {
            clip_id: ClipId::new(DbId::from_str(&clip.clip_id)?),
            content: Content::new(clip.content.as_str())?,
            expires: Expires::new(clip.expires.map(Time::from_naive_utc)),
            hits: Hits::new(u64::try_from(clip.hits)?),
            password: Password::new(clip.password.unwrap_or_default())?,
            posted: Posted::new(Time::from_naive_utc(clip.posted)),
            shortcode: ShortCode::from(clip.shortcode),
            title: Title::new(clip.title),
        })
    }
}

pub struct GetClip {
    pub(in crate::data) shortcode: String,
}

impl From<crate::service::ask::GetClip> for GetClip {
    fn from(req: crate::service::ask::GetClip) -> Self {
        Self {
            shortcode: req.shortcode.into_inner(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode: shortcode.into_inner(),
        }
    }
}

impl From<String> for GetClip {
    fn from(shortcode: String) -> Self {
        Self { shortcode }
    }
}

pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: i64,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<ask::NewClip> for NewClip {
    fn from(req: ask::NewClip) -> Self {
        Self {
            clip_id: DbId::default().into(),
            shortcode: ShortCode::default().into(),
            content: req.content.into_inner(),
            expires: req.expires.into_inner().map(|time| time.timestamp()),
            password: req.password.into_inner(),
            title: req.title.into_inner(),
            posted: Utc::now().timestamp(),
        }
    }
}

pub struct UpdateClip {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<ask::UpdateClip> for UpdateClip {
    fn from(req: ask::UpdateClip) -> Self {
        Self {
            title: req.title.into_inner(),
            content: req.content.into_inner(),
            expires: req.expires.into_inner().map(|time| time.timestamp()),
            password: req.password.into_inner(),
            shortcode: req.shortcode.into_inner(),
        }
    }
}
