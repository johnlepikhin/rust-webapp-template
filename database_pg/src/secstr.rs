use diesel::deserialize::FromSql;
use diesel::sql_types::Text;
use diesel::{backend::Backend, deserialize};
use diesel::{serialize, AsExpression, Queryable};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, AsExpression)]
#[diesel(sql_type = Text)]
pub struct SecUtf8(secstr::SecUtf8);

impl From<secstr::SecUtf8> for SecUtf8 {
    fn from(v: secstr::SecUtf8) -> Self {
        Self(v)
    }
}

impl From<String> for SecUtf8 {
    fn from(v: String) -> Self {
        Self(secstr::SecUtf8::from(v))
    }
}

impl Deref for SecUtf8 {
    type Target = secstr::SecUtf8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SecUtf8> for secstr::SecUtf8 {
    fn from(val: SecUtf8) -> Self {
        val.0
    }
}

impl<B: Backend> serialize::ToSql<Text, B> for SecUtf8
where
    str: serialize::ToSql<Text, B>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, B>) -> serialize::Result {
        <str as serialize::ToSql<Text, B>>::to_sql(self.unsecure(), out)
    }
}

impl<B: Backend> FromSql<Text, B> for SecUtf8
where
    String: FromSql<Text, B>,
{
    fn from_sql(bytes: B::RawValue<'_>) -> deserialize::Result<Self> {
        <String as FromSql<Text, B>>::from_sql(bytes)
            .map(|v| SecUtf8::from(secstr::SecUtf8::from(v)))
    }
}

impl<DB> Queryable<Text, DB> for SecUtf8
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    type Row = String;

    fn build(s: String) -> deserialize::Result<Self> {
        Ok(SecUtf8::from(s))
    }
}
