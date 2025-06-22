/// a struct repesenting the database with the name 'primary'

// pub use super::db::_url::Url;
pub struct Url(url::Url);
impl std::ops::Deref for self::Url {
    type Target = url::Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<url::Url> for self::Url {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
impl std::convert::From<url::Url> for self::Url {
    fn from(value: url::Url) -> Self {
        Self(value)
    }
}
use rocket::form::{self, ValueField};

impl rocket::http::uri::fmt::UriDisplay<rocket::http::uri::fmt::Query> for self::Url {
    fn fmt(
        &self,
        f: &mut rocket::http::uri::fmt::Formatter<'_, rocket::http::uri::fmt::Query>,
    ) -> std::fmt::Result {
        use std::fmt::Write;
        let d = urlencoding::encode(self.0.as_str());
        f.write_str(&d)
    }
}

// when you pass a string as a parameter to the url, it will attempt to decode the url before parsing it

impl rocket::http::uri::fmt::FromUriParam<rocket::http::uri::fmt::Query, String> for self::Url {
    type Target = Result<Self, anyhow::Error>;
    fn from_uri_param(param: String) -> Self::Target {
        let decoded = urlencoding::decode(&param)?;
        Ok(Self(url::Url::parse(&decoded)?))
    }
}
// when you pass a url as a parameter it will pass it through unchanged

impl rocket::http::uri::fmt::FromUriParam<rocket::http::uri::fmt::Query, Self> for self::Url {
    type Target = Result<Self, anyhow::Error>;
    fn from_uri_param(param: Self) -> Self::Target {
        // let decoded = urlencoding::decode(&param)?;
        Ok(param)
    }
}
#[rocket::async_trait]
impl<'r> rocket::form::FromFormField<'r> for self::Url {
    fn from_value(field: ValueField<'_>) -> form::Result<'_, Self> {
        let decoded = match urlencoding::decode(field.value) {
            Ok(v) => v,
            Err(e) => {
                let mut errors = rocket::form::Errors::new();
                errors = errors.with_name(field.name);
                errors.push(rocket::form::Error::validation(e.to_string()));
                return rocket::form::Result::Err(errors);
            }
        };
        match url::Url::parse(&decoded) {
            Ok(url) => Ok(Self(url)),
            Err(e) => {
                let mut errors = rocket::form::Errors::new();
                errors = errors.with_name(field.name);
                errors.push(rocket::form::Error::validation(e.to_string()));
                return rocket::form::Result::Err(errors);
            }
        }
    }
}
