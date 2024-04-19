use serde::{Deserialize, Serialize};

// @<claims
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub iat: i64,
    pub exp: i64,
}
// >@
