mod sql_request_sqlite;
mod sql_request;

/*Module "appdyn.rs" qui contient le code du lancement du serveur Web avec tide */
pub mod appdyn {
    use sqlx::mysql::MySqlPoolOptions;
    use tide::{Body, Request, Response, Server};
    use crate::appdyn::sql_request::sql_request::{get_data, json_to_html};
    extern crate urlencoding;
    use urlencoding::decode;
    use super::sql_request::sql_request::set_data;

    #[derive(Clone, Debug)]
    struct State {
        pool: sqlx::Pool<sqlx::MySql>,
        actions: serde_json::value::Value, //Arc<RwLock<HashMap<String, String>>>,
    }

    #[async_std::main]
    pub async fn main() {
        tide::log::start();
        let pool = MySqlPoolOptions::new()
        .max_connections(7)
        .connect("mysql://admin:azerty@localhost/greta")
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

        // Variables accessibles depuis n'importe quel fonction de Tide. : Contexte.
        let state = State {
            pool: conn.clone(),
            actions: config_actions,
        };

        let mut app = tide::with_state(state);

        app.at("/").get(|_| async { Ok("Hello, world!") });

        app.at("/getview/:view")
            .get(|mut _req: Request<State>| async move {
                let action = get_action(&_req, "GET/getview/".to_string()).await;
                // Récupération de la vue via la variable "view" du fichier config_actions.json chargé au démarrage
                let view = action.get("view").unwrap().as_str().unwrap();
                let sql_action = action.get("sql_action").unwrap().as_str().unwrap();
                // problème dans la construction du json, voici le résultat de j :
                // "\n                {\n                    \"sql_action\": \"\"insert\"\"\n                }"
                let j = "{ \"sql_action\": \"?\"}";
                let jsontxt = j.replace('?',sql_action);
                println!("jsontxt : {:?}", jsontxt);
                let mut json: Vec<serde_json::Value> = Vec::new();
                json.push(serde_json::from_str(jsontxt.as_str()).unwrap()); // JSON pas conforme à la norme RFC mais convertible
                let html = json_to_html(&json, view).unwrap();  // utilsation de handlebars pour tranformatin en HTML
                let mut res = Response::new(200);
                res.set_content_type("text/html");
                res.set_body(Body::from_string(html));
                Ok(res)
            });

        app.at("/getdata/:table")
            .get(|mut _req: Request<State>| async move {
                let action = get_action(&_req, "GET/getdata/".to_string()).await;
                // convertir le String(....) en str avec : serde_json::Value::as_str(action.get("sql").unwrap())
                let sql_query = serde_json::Value::as_str(action.get("sql").unwrap()).unwrap();
                let view = serde_json::Value::as_str(action.get("view").unwrap()).unwrap();
                let response = get_data(&_req.state().pool, &sql_query.to_string()).await;
                let mut json: Vec<serde_json::Value> = Vec::new();
                for row in response.unwrap() {
                    json.push(serde_json::from_str(&row).unwrap()); // JSON pas conforme à la norme RFC mais convertible
                }
                // afficher le retour du parsing handlabars sur la Sortie std
                let html = json_to_html(&json, view).unwrap();  // utilsation de handlebars pour tranformatin en HTML

                let mut res = Response::new(200);
                res.set_content_type("text/html");
                res.set_body(Body::from_string(html));
                Ok(res)
            });

            app.at("/setdata/:table")
            .get(|mut _req: Request<State>| async move {
                println!("\n===> req.body :dans get");
                // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
                println!("\n===> req.query : {:?}", _req.url().query().unwrap().split('&'));
                let url_params = _req.url().query().unwrap().split('&')
                .filter_map(|s| {
                    s.split_once('=')
                        .and_then(|t| Some((decode(t.0.to_owned().as_str()).unwrap(), decode(t.1.to_owned().replace("+", " ").as_str()).unwrap())))
                })
                .collect::<Vec<_>>();
                let mut vec_keys: Vec<String> = Vec::new();
                println!("\n===> req.query : {:?}", url_params);
                for p in url_params {
                    vec_keys.push(p.1);
                }
                let action = get_action(&_req, "GET/setdata/".to_string()).await;
                println!("\n===> action: {:?}", action);
                // convertir le String(....) en str avec : serde_json::Value::as_str(action.get("sql").unwrap())
                let sql_query = serde_json::Value::as_str(action.get("sql").unwrap()).unwrap();
                let response = set_data(&_req.state().pool, sql_query.to_string(), vec_keys).await;
                                
                let mut res = Response::new(200);
                res.set_content_type("text/html");
                res.set_body(Body::from_string(format!("<H1>{} Row inserted !</H1>", response)));
                Ok(res)
            })
            .post(|mut _req: Request<State>| async move {
                // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
                let body =_req.body_string().await?;
                let url_params = body.split('&')
                .filter_map(|s| {
                    s.split_once('=')
                        .and_then(|t| Some((decode(t.0.to_owned().as_str()).unwrap(), decode(t.1.to_owned().replace("+", " ").as_str()).unwrap())))
                })
                .collect::<Vec<_>>();
                let mut vec_keys: Vec<String> = Vec::new();
                println!("\n===> params : {:?}", url_params);
                for p in url_params {
                    vec_keys.push(p.1);
                }
                let action = get_action(&_req, "POST/setdata/".to_string()).await;
                println!("\n===> action: {:?}", action);
                // convertir le String(....) en str avec : serde_json::Value::as_str(action.get("sql").unwrap())
                let sql_query = action.get("sql").unwrap().as_str().unwrap();
                let response = set_data(&_req.state().pool, sql_query.to_string(), vec_keys).await;
                                
                let mut res = Response::new(200);
                res.set_content_type("text/html");
                res.set_body(Body::from_string(format!("<H1>{} Row inserted !</H1>", response)));
                Ok(res)
            });

            app.at("/updatedata/:table")
            .get(|mut _req: Request<State>| async move {
                // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
                let action = get_action(&_req, "GET/updatedata/".to_string()).await;
                // convertir le String(....) en str avec : serde_json::Value::as_str(action.get("sql").unwrap())
                let sql_query = action.get("sql").unwrap().as_str().unwrap();
                let body =_req.body_string().await?;
                let url_params = body.split('&')
                .filter_map(|s| {
                    s.split_once('=')
                        .and_then(|t| Some((decode(t.0.to_owned().as_str()).unwrap(), decode(t.1.to_owned().replace("+", " ").as_str()).unwrap())))
                })
                .collect::<Vec<_>>();
                let mut vec_keys: Vec<String> = Vec::new();
                println!("\n===> params : {:?}", url_params);
                for p in url_params {
                    vec_keys.push(p.1);
                }
                let response = set_data(&_req.state().pool, sql_query.to_string(), vec_keys).await;
                                
                let mut res = Response::new(200);
                res.set_content_type("text/html");
                res.set_body(Body::from_string(format!("<H1>{} Row updated !</H1>", response)));
                Ok(res)
            });

            app.at("/deletedata/:table")
            .get(|mut _req: Request<State>| async move {
                // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
                let action = get_action(&_req, "GET/deletedata/".to_string()).await;
                // convertir le String(....) en str avec : serde_json::Value::as_str(action.get("sql").unwrap())
                let sql_query =action.get("sql").unwrap().as_str().unwrap();
                let body =_req.body_string().await?;
                let url_params = body.split('&')
                .filter_map(|s| {
                    s.split_once('=')
                        .and_then(|t| Some((decode(t.0.to_owned().as_str()).unwrap(), decode(t.1.to_owned().replace("+", " ").as_str()).unwrap())))
                })
                .collect::<Vec<_>>();
                let mut vec_keys: Vec<String> = Vec::new();
                println!("\n===> params : {:?}", url_params);
                for p in url_params {
                    vec_keys.push(p.1);
                }
                let response = set_data(&_req.state().pool, sql_query.to_string(), vec_keys).await;
                                
                let mut res = Response::new(200);
                res.set_content_type("text/html");
                res.set_body(Body::from_string(format!("<H1>{} Row deleted !</H1>", response)));
                Ok(res)
            })
            .post(|mut _req: Request<State>| async move {
                println!("\n===> req.body :dans post");
                let body =_req.body_string().await?;
                println!("\n===> req.body : {:?} \n===> req.body_json", body);
                let url_params = body.split('&')
                .filter_map(|s| {
                    s.split_once('=')
                        .and_then(|t| Some((decode(t.0.to_owned().as_str()).unwrap(), decode(t.1.to_owned().replace("+", " ").as_str()).unwrap())))
                })
                .collect::<Vec<_>>();
                let mut vec_keys: Vec<String> = Vec::new();
                println!("\n===> url_params : {:?}", url_params);
                for p in url_params {
                    vec_keys.push(p.1);
                }
                let action = get_action(&_req, "GET/deletedata/".to_string()).await;
                let sql_query = action.get("sql").unwrap().as_str().unwrap();
                let response = set_data(&_req.state().pool, sql_query.to_string(), vec_keys).await;
                                
                let mut res = Response::new(200);
                res.set_content_type("text/html");
                res.set_body(Body::from_string(format!("<H1>{} Row deleted !</H1>", response)));
                Ok(res)
            });
        app
    }

    async fn get_action(_req: &Request<State>, prefix_cle: String) -> serde_json::value::Value{
        // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
        let mut cle = prefix_cle.to_string();
        if prefix_cle.contains("getview") {
            cle.push_str(_req.param("view").unwrap());
        }
        else {
            cle.push_str(_req.param("table").unwrap());
        }
        println!("\n===>cle : {:?}", cle);
        _req.state().actions.get(&cle).unwrap().clone()
    }
}
