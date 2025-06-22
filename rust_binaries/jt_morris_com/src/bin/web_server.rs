#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use futures_util::TryFutureExt;
    use jt_morris_dot_com::web_server::config::DatabaseConfig;
    use rocket::time;
    // defining this function inside the lib allows it to be reused for tests
    jt_morris_dot_com::web_server::db::initialize_rustls();
    let rocket = jt_morris_dot_com::web_server::rocket().await?;
    // rocket.figment().extract_inner::<HashMap<String,rocket_db_pools::Config>>("databases.primary");

    let rocket = rocket.ignite().await?;

    // let (s,mut r) = tokio::sync::mpsc::unbounded_channel();
    let figment = rocket.figment().to_owned();
    let (tx, mut rx) = rocket::tokio::sync::mpsc::channel::<()>(1);
     rocket::tokio::task::spawn(async move {
        let config = figment
            .extract_inner::<DatabaseConfig>("databases.primary")
            .unwrap();
        let mut connection = jt_morris_dot_com::web_server::db::establish_connection(
            config.url.as_str(),
            // config.max_connections,
        )
        .await
        .unwrap();
        // let mut conn = pool.get().unwrap();
        loop {
            let _ = jt_morris_dot_com::web_server::db::auth_state::AuthState::delete_older_than_duration(
                &mut connection,
                time::Duration::minutes(15),
            ).map_err(|e| {
                tracing::error!("Autodeleting of old auth_state failed: {e}")
            });
            rocket::tokio::select! {
                _ =rocket::tokio::time::sleep(std::time::Duration::from_secs(120)) =>{},
                _ = rx.recv() => break

            }
        }
    });
    let launch = rocket.launch().await;
   

    // let _ = s.send(()).ok();
    let _ = launch?;
    tx.send(()).await.ok();
    Ok(())
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn main() {}
