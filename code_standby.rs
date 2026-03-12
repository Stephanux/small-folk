/* app.at("/").get(|_| async { Ok("Hello, world!") });

        app.at("/getview/:view")
            .get(|mut _req: Request<State>| async move {
                let action = get_action(&_req, "GET/getview/".to_string()).await;
                // Récupération de la vue via la variable "view" du fichier config_actions.json chargé au démarrage
                let view = action.get("view").unwrap().as_str().unwrap();
                let form_action = action.get("form_action").unwrap().as_str().unwrap();
                // problème dans la construction du json, voici le résultat de j :
                // "\n                {\n                    \"sql_action\": \"\"insert\"\"\n                }"
                let j = "{ \"sql_action\": \"?\"}";
                let jsontxt = j.replace('?', form_action);
                println!("jsontxt : {:?}", jsontxt);
                let mut json: Vec<serde_json::Value> = Vec::new();
                json.push(serde_json::from_str(jsontxt.as_str()).unwrap()); // JSON pas conforme à la norme RFC mais convertible
                let html = json_to_html(&json, view).unwrap(); // utilisation de handlebars pour transformation en HTML
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
                // Traite l'erreur possible en retour de la function get_data() via "is_ok()"" de response de type Result.
                let mut json: Vec<serde_json::Value> = Vec::new();
                if response.is_ok() {
                    for row in response.unwrap() {
                        json.push(serde_json::from_str(&row).unwrap()); // JSON pas conforme à la norme RFC mais convertible
                    }
                } else {
                    let mut j: String = String::new();
                    j.push_str("{ \"Error\": \"Database Error\", \"sql_query\": \"");
                    j.push_str(&format!("{}", sql_query));
                    j.push_str("\" }");
                    json.push(serde_json::from_str(j.as_str()).unwrap());
                }
                // afficher le retour du parsing handlabars sur la Sortie std
                let html = json_to_html(&json, view).unwrap(); // utilisation de handlebars pour transformation en HTML

                let mut res = Response::new(200);
                res.set_content_type("text/html");
                res.set_body(Body::from_string(html));
                Ok(res)
            });

        app.at("/setdata/:table")
            .get(|mut _req: Request<State>| async move {
                println!("\n===> req.body :dans get");
                // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
                println!(
                    "\n===> req.query : {:?}",
                    _req.url().query().unwrap().split('&')
                );
                let url_params = _req
                    .url()
                    .query()
                    .unwrap()
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
                res.set_body(Body::from_string(format!(
                    "<H1>{} Row inserted !</H1>",
                    response
                )));
                Ok(res)
            })
            .post(|mut _req: Request<State>| async move {
                // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
                let body = _req.body_string().await?;
                let url_params = body
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
                res.set_body(Body::from_string(format!(
                    "<H1>{} Row inserted !</H1>",
                    response
                )));
                Ok(res)
            });

        app.at("/updatedata/:table")
            .get(|mut _req: Request<State>| async move {
                // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
                let action = get_action(&_req, "GET/updatedata/".to_string()).await;
                // convertir le String(....) en str avec : serde_json::Value::as_str(action.get("sql").unwrap())
                let sql_query = action.get("sql").unwrap().as_str().unwrap();
                //let body =_req.body_string().await?;
                let url_params = _req
                    .url()
                    .query()
                    .unwrap()
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
                let mut vec_keys: Vec<String> = Vec::new();
                println!("\n===> params : {:?}", url_params);
                for p in url_params {
                    vec_keys.push(p.1);
                }
                let response = set_data(&_req.state().pool, sql_query.to_string(), vec_keys).await;

                let mut res = Response::new(200);
                res.set_content_type("text/html");
                res.set_body(Body::from_string(format!(
                    "<H1>{} Row updated !</H1>",
                    response
                )));
                Ok(res)
            });

        app.at("/deletedata/:table")
            .get(|mut _req: Request<State>| async move {
                // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
                let action = get_action(&_req, "GET/deletedata/".to_string()).await;
                // convertir le String(....) en str avec : serde_json::Value::as_str(action.get("sql").unwrap())
                let sql_query = action.get("sql").unwrap().as_str().unwrap();
                let _body = _req.body_string().await?;
                let url_params = _req
                    .url()
                    .query()
                    .unwrap()
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
                let mut vec_keys: Vec<String> = Vec::new();
                println!("\n===> params : {:?}", url_params);
                for p in url_params {
                    vec_keys.push(p.1);
                }
                let response = set_data(&_req.state().pool, sql_query.to_string(), vec_keys).await;

                let mut res = Response::new(200);
                res.set_content_type("text/html");
                res.set_body(Body::from_string(format!(
                    "<H1>{} Row deleted !</H1>",
                    response
                )));
                Ok(res)
            })
            .post(|mut _req: Request<State>| async move {
                println!("\n===> req.body :dans post");
                let body = _req.body_string().await?;
                println!("\n===> req.body : {:?} \n===> req.body_json", body);
                let url_params = body
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
                res.set_body(Body::from_string(format!(
                    "<H1>{} Row deleted !</H1>",
                    response
                )));
                Ok(res)
            }); */