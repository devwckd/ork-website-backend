use std::fmt::{Debug, Display};

#[inline]
pub fn handle_sqlx_unique<E: Debug>(
    sqlx_err: sqlx::Error,
    constraint_name: &str,
    matches: fn(String) -> E,
    otherwise: fn(String) -> E,
) -> E {
    match sqlx_err {
        sqlx::Error::Database(error) => {
            if error.message().contains(constraint_name) {
                matches(error.to_string())
            } else {
                otherwise(error.to_string())
            }
        }
        _ => otherwise(sqlx_err.to_string()),
    }
}
