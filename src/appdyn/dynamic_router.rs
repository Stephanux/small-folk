/*#[derive(Clone, Debug)]
 pub struct Message {
    action: String,
    method: String,
} */

pub mod dynamic_router {
    use tide::{Body, Request, Response, Server};
    extern crate urlencoding;
    use crate::appdyn;
    use crate::appdyn::appdyn::State;
    use crate::appdyn::{
        sql_request::sql_request::get_data, sql_request::sql_request::set_data,
        view::view::json_to_html,
    };
    use urlencoding::decode;

    pub async fn manage_action<'a>(app: &'a mut Server<State>, ) -> &'a mut Server<appdyn::appdyn::State> {
        /* ici on va découper les requêtes et récupérer les données qui arrive du client  */
        app.at("*").get(|mut _req: Request<State>| async move {
            println!("\n===> path : {:?}", _req.url().path());
            let url_params = _req
                .url()
                .query()
                .unwrap_or("")
                .split('&')
                .filter_map(|s| {
                    s.split_once('=').and_then(|t| {
                        Some((
                            decode(t.0.to_owned().as_str()).unwrap(),
                            decode(t.1.to_owned().replace("+", " ").as_str()).unwrap(),
                        ))
                    })
                })
                .collect::<Vec<_>>();
            println!("\n===> url_params : {:?}", url_params);
            let body = _req.body_string().await.unwrap_or("".to_string());
            let methode = _req.method().to_string();
            println!("\n===> body : {:?} \n===> method : {:?}", body, methode);
            let mut res = Response::new(200);
            res.set_content_type("text/html");
            res.set_body(Body::from_string(methode+_req.url().path()+"?"+url_params.iter().map(|(k,v)| format!("{}={}&", k, v)).collect::<String>().as_str()));
            Ok(res)
        });

        app
    }

    async fn get_action(_req: &Request<State>, prefix_cle: String) -> serde_json::value::Value {
        // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
        let mut cle = prefix_cle.to_string();
        println!("cle : {:?}", cle);
        if prefix_cle.contains("getview") || prefix_cle.contains("getupdateview") {
            cle.push_str(_req.param("view").unwrap());
        } else {
            cle.push_str(_req.param("table").unwrap());
        }
        println!("\n===>cle : {:?}", cle);
        if _req.state().actions.get(&cle) == None {
            let val = _req.state().actions.get("GET/notfound");
            return val.unwrap().clone();
        } else {
            _req.state().actions.get(&cle).unwrap().clone()
        }
    }
}
