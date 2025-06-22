
/// a struct repesenting the database with the name 'primary'

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),derive(diesel::AsExpression))]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(sql_type=diesel::sql_types::Text))]


// #[cfg_attr(feature="debug", derive(Debug))]
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Url(pub(crate) url::Url);
impl std::convert::From<url::Url> for self::Url {
    fn from(value: url::Url) -> Self {
        Self(value)
    }
}
// when you pass a string as a parameter to the url, it will attempt to decode the url before parsing it

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
    fn from(v: url::Url) -> Self {
        Self(v)
    }
}
#[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::pg::Pg> for self::Url {
    fn from_sql(value: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
        let bytes = value.as_bytes();
        let string = String::from_utf8_lossy(bytes);
        match url::Url::parse(&string) {
            Ok(u) => diesel::deserialize::Result::Ok(Self(u)),
            Err(e) => diesel::deserialize::Result::Err(e.into()),
        }
    }
}
#[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg> for self::Url {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        let mut str = self.0.as_str();
        diesel::serialize::ToSql::<diesel::sql_types::Text,diesel::pg::Pg>::to_sql(&mut str,&mut out.reborrow())
    }
}

