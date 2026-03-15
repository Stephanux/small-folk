//mod sql_request_sqlite;
mod dynamic_router;
mod sql_request;
mod view;

/*Module "appdyn.rs" qui contient le code du lancement du serveur Web avec Tide */
pub mod appdyn {
    use crate::appdyn::dynamic_router::dynamic_router::manage_action;
    use sqlx::mysql::MySqlPoolOptions;
    use tide::{Server};
    use core::State;
    extern crate urlencoding;

    #[async_std::main]
    pub async fn main() {
        tide::log::start();
        let pool = MySqlPoolOptions::new()
            .max_connections(7)
            .connect("mysql://admin:azerty@localhost/R504TP")
            .await;
        let conn = pool.unwrap().clone();

        println!("Connection database {:?}", conn);
        let app = server(&conn).await;

        app.listen("127.0.0.1:8080").await.unwrap();
    }

    async fn server(conn: &sqlx::Pool<sqlx::MySql>) -> Server<State> {
        let config_actions = {
            // Load the routes configuration file into a string.
            let text = std::fs::read_to_string("./config_actions.json").unwrap();
            //println!("\n==> test json read : {:#?}", text);
            // Parse the string into a dynamically-typed JSON structure.
            serde_json::from_str::<serde_json::value::Value>(&text).unwrap()
        };
        // Variables accessibles depuis n'importe quelle fonction de Tide. : Contexte.
        let state = State {
            pool: conn.clone(),
            actions: config_actions,
        };
        // on prépare les donnée dans State et on appelle "manage_action" pour traiter la requête HTTP
        let mut app = tide::with_state(state);
        // Gestion de l'action dans le dynamic_router.rs et la fonction manage_action(&app)
        manage_action(&mut app).await; 

        app
    }
}
