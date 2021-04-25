use itertools::Itertools;
use rocket::http::RawStr;
use rocket::request::FromFormValue;

/// カンマ区切りで複数の値を指定する文字列型クエリ
#[derive(Debug)]
pub struct CommaSeparatedValues(Vec<String>);

impl<'v> FromFormValue<'v> for CommaSeparatedValues {
    type Error = &'v RawStr;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        match v.url_decode() {
            Ok(decoded) => Ok(CommaSeparatedValues(
                decoded.split(',').map(String::from).collect_vec(),
            )),
            _ => Err(v),
        }
    }
}

impl CommaSeparatedValues {
    pub fn unwrap(self) -> Vec<String> {
        self.0
    }
}
