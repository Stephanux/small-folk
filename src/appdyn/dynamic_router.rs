/*#[derive(Clone, Debug)]
 pub struct Message {
    action: String,
    method: String,
} */

pub mod dynamic_router {
    use tide::{Body, Request, Response, Server};
    extern crate urlencoding;
    use crate::appdyn;
    use core::State;
    use crate::appdyn::{
        sql_request::sql_request::get_data, sql_request::sql_request::set_data,
        view::view::json_to_html,
    };
    use urlencoding::decode;

    use libloading::Library;

    use core::{Plugin, PluginRegistrar};

    struct Registrar {
        plugins: Vec<Box<dyn Plugin + Send>>,
    }

    impl PluginRegistrar for Registrar {
        fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
            // SAFETY: We require plugins to be Send for async handler compatibility
            let plugin = unsafe {
                // Transmute Box<dyn Plugin> to Box<dyn Plugin + Send>
                std::mem::transmute::<Box<dyn Plugin>, Box<dyn Plugin + Send>>(plugin)
            };
            self.plugins.push(plugin);
        }
    }

    pub async fn manage_action<'a>(app: &'a mut Server<State>, ) -> &'a mut Server<core::State> {
        
        /* ici on va découper les requêtes et récupérer les données qui arrive du client  */
        app.at("*").get(|mut _req: Request<State>| async move {
            let mut registrar = Registrar {
                plugins: Vec::new(),
            };
            let action = get_action(&_req, "GET/plugins/plugin_a".to_string()).await;
            let path = action.get("plugin").unwrap().as_str().unwrap();
            let mut res = Response::new(200);
            unsafe {
                // In this code, we never close the shared library - if you need to be able to unload the
                // library, that will require more work.
                let lib = Box::leak(Box::new(Library::new(path).unwrap()));
                // NOTE: You need to do something to ensure you're only loading "safe" code. Out of scope
                // for this code.
                let func: libloading::Symbol<unsafe extern "C" fn(&mut dyn PluginRegistrar) -> ()> = lib.get(b"plugin_entry").unwrap();
                func(&mut registrar);
            }
            for plugin in registrar.plugins {
                plugin.callback1();
                let response = plugin.execute(&_req);
                match response {
                    Ok(r) => res = r,
                    Err(e) => {
                        res.set_status(500);
                        res.set_body(format!("Plugin execution error: {}", e));
                    }
                }
                dbg!(plugin.callback2(7));
            }
            /*println!("\n===> path : {:?}", _req.url().path());*/            
            Ok(res)
        });
        app
    }

    async fn get_action(_req: &Request<State>, prefix_cle: String) -> serde_json::value::Value {
        // récupération des données de l'URL : pathname + nom table param + sql from config_action.json
        let cle = prefix_cle.to_string();
        println!("cle : {:?}", cle);
        _req.state().actions.get(&cle).unwrap().clone()
       /* if prefix_cle.contains("getview") || prefix_cle.contains("getupdateview") {
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
        }*/
    }
}
