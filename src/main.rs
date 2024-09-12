#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::sync::Arc;
    use actix_files::Files;
    use actix_identity::{config::LogoutBehaviour, IdentityMiddleware};
    use actix_session::{config::PersistentSession, SessionMiddleware};
    use actix_web::*;
    use cookie::{time::Duration, Key};
    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_meta::MetaTags;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use beu::{actix_session_surrealdb::SurrealSessionStore, app::*, config::AppConfig, db::db::{ConnectionOptions, DB}};
    use middleware::Compress;
    use once_cell::sync::Lazy;
    use surrealdb::opt::auth::Root;
    use surrealdb_migrations::MigrationRunner;
    use web::Data;
    use actix_cors::Cors;
    use dotenvy::dotenv;

    dotenv().expect("Error Loading Environment Variables");

    static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::init());

    let namespace = &APP_CONFIG.database_namespace;
    let database = &APP_CONFIG.database_name;
    let username = &APP_CONFIG.database_username;
    let endpoint = &APP_CONFIG.database_endpoint;
    let password = &APP_CONFIG.database_password;

    let conn_opts = ConnectionOptions {
        namespace,
        database,
        credentials: Root { username, password },
    };

    let db = DB::connect(&endpoint, &conn_opts)
        .await
        .unwrap_or_else(|err| {
            println!("Error Connecting To SurrealDB");
            println!("{}", err);
            std::process::exit(1);
        });

        MigrationRunner::new(&db)
        .up()
        .await
        .expect("Failed to apply migrations");
    
    let session_db = DB::connect(&endpoint, &conn_opts)
    .await
    .unwrap_or_else(|err| {
        println!("Error Connecting To SurrealDB");
        println!("{}", err);
        std::process::exit(1);
    });
    
    let db_ctx = Data::new(db.clone()); 

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        let json_config = web::JsonConfig::default()
            .limit(20480)
            .error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
            });

        let cors = Arc::new(
            Cors::default()
                .allowed_origin(&APP_CONFIG.domain)
                .supports_credentials()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600),
        );
        let auth_cookie_key = if APP_CONFIG.auth_cookie_key.is_empty() {
            Key::generate()
        } else {
            Key::from(APP_CONFIG.auth_cookie_key.clone().as_bytes())
        };
        
       

        println!("listening on http://{}", &addr);

        App::new()
            .wrap(IdentityMiddleware::builder().logout_behaviour(LogoutBehaviour::PurgeSession).build())
            .wrap(Compress::default())
            .wrap(
                SessionMiddleware::builder(
                    SurrealSessionStore::from_connection(session_db.clone(), "sessions"),
                    auth_cookie_key,
                )
                .cookie_same_site(actix_web::cookie::SameSite::None)
                .cookie_secure(true)
                .cookie_http_only(true)
                .cookie_domain(APP_CONFIG.cookie_domain.clone())
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl_extension_policy(actix_session::config::TtlExtensionPolicy::OnStateChanges)
                        .session_ttl(Duration::days(7)),
                )
                .build(),
            )
            .wrap(cors)
            .app_data(json_config)
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root))
            .service(favicon)
            .app_data(db_ctx.clone())
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}
