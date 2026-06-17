#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::get;
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use voice_base::app::*;
    use voice_base::axum_sandbox::hello_axum_sandbox;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes_leptos = generate_route_list(App);

    let axum_api_sandbox = Router::new().route(
        "/axum_sb",
        get(hello_axum_sandbox),
    );

    // INFO: what I see in examples is not clear for me right mow how could i share connection to the db in server, or should I in the first place?
    // INFO: for now let's create new connection every time needed.
    // let _db_pool = sqlx::PgPool::connect("postgres://postgres:do_change_me_later_123/localhost/voidebase").await.expect("Cannot connect to DB");
    // let mut conn = db().await.expect("couldn't connect to DB");

    let app = Router::new()
        .merge(axum_api_sandbox) // INFO: I can merge other routes like this. see oalso .nest
        .leptos_routes(&leptos_options, routes_leptos, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
