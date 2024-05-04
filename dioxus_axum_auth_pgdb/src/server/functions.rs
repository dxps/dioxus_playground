use dioxus_fullstack::prelude::*;

#[cfg(feature = "server")]
use sqlx::{postgres::PgRow, Row};

#[server(Login)]
pub async fn login() -> Result<(), ServerFnError> {
    let auth: crate::auth::Session = extract().await?;
    auth.login_user(2);
    Ok(())
}

#[server(Logout)]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth: crate::auth::Session = extract().await?;
    auth.logout_user();
    Ok(())
}

#[server(GetUserName)]
pub async fn get_user_name() -> Result<String, ServerFnError> {
    let session: crate::auth::Session = extract().await?;

    // Interacting with the database.
    let dbc = session.1;
    sqlx::query("SELECT version() as version;")
        .map(|r: PgRow| {
            log::debug!("Database version: {}", r.get::<String, _>("version"));
        })
        .fetch_all(dbc.as_ref())
        .await;

    Ok(session.0.current_user.unwrap().username.to_string())
}

#[server(Permissions)]
pub async fn get_permissions() -> Result<String, ServerFnError> {
    let method: axum::http::Method = extract().await?;
    let auth: crate::auth::Session = extract().await?;
    let curr_user = auth.current_user.clone().unwrap_or_default();

    // Lets check permissions only and not worry about if they are anonymous or not.
    if !axum_session_auth::Auth::<crate::auth::User, i64, sqlx::PgPool>::build([axum::http::Method::POST], false)
        .requires(axum_session_auth::Rights::any([
            axum_session_auth::Rights::permission("Category::View"),
            axum_session_auth::Rights::permission("Admin::View"),
        ]))
        .validate(&curr_user, &method, None)
        .await
    {
        return Ok(format!(
            "User '{}' does not have permissions needed to view this page. Please login.",
            curr_user.username
        ));
    }

    Ok(format!(
        "User '{}' has the permissions to view this page. Here are his permissions: {:?}",
        curr_user.username, curr_user.permissions
    ))
}
