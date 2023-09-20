/* Ce module permettra de récupérer les données de la base et les informations de la requête 
pour appeler la vue correspondante définie dans le fichier config_actions.json */

pub mod view {
    use std::collections::BTreeMap;
    use handlebars::Handlebars;

    /********************************************************************************************** */
    /*-- Méthode json_to_html() : permet de générer une page html à partir d'un template Handlebars */
    /********************************************************************************************** */
    pub fn json_to_html(data: &Vec<serde_json::Value>, view: &str) -> Result<String, handlebars::RenderError> {
        let mut handlebars = Handlebars::new();
        let _resp = handlebars
            .register_template_file("datahbs", view)
            .unwrap();
        // ici gerer le retour du parsing hbs via "_resp" et tester si error... match _resp { ...un truc du genre
        let mut datahbs: BTreeMap<String, Vec<serde_json::Value>> = BTreeMap::new();
        datahbs.insert("data".to_string(), data.clone());
        println!("\n==> handlebars : {:?}", handlebars.render_template("datahbs", &datahbs));
        handlebars.render("datahbs", &datahbs)
    }
}