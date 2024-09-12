#[cfg(feature = "ssr")]
pub use crate::error::AppError;

/// Macro that returns a 500 Internal Server Error JSON Response
/// Can take a string literal as an argument to change the error message
///
/// ## Usage
/// ```no_run
/// internalError!() // Returns a "Internal Server Error" with code 500
/// internalError!("Database Error") // Returns a "Database Error" with code 500
/// ```
#[macro_export]
#[cfg(feature = "ssr")]
macro_rules! internalError {
    ($l:literal) => {{
        return Ok(actix_web::web::Json($crate::api::response::Response::new_error(500, $l.to_string())));
    }};
    () => {{
        return Ok(actix_web::web::Json($crate::api::response::Response::new_error(
            500,
            "Internal Server Error".to_string(),
        )));
    }};
}

#[macro_export]
#[cfg(feature = "ssr")]
macro_rules! map_err {
    ($result:expr, $err_type:expr) => {{
        let mapped: Result<_, crate::error::AppError;> = $result.map_err(|err| $err_type(err));
        mapped
    }};
    (DBErr -> $result:expr) => {{
        $result.map_err(|err| crate::error::AppError::DatabaseError(err))
    }};

    (ServerErr -> $result:expr) => {{
        $result.map_err(|err| crate::error::AppError::InternalError(err))
    }};

    (IoErr -> $result:expr) => {{
        $result.map_err(|err| crate::error::AppError::IOError(err))
    }};
}

#[macro_export]
#[cfg(feature = "ssr")]
macro_rules! data_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map: ::std::collections::BTreeMap<String, surrealdb::sql::Value>  = ::std::collections::BTreeMap::new();

        $(map.insert($key.into(), $value);)+
        map
    }};
}