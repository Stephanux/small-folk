pub mod sql_request {
    use handlebars::Handlebars;
    use serde_json::{self};
    use std::collections::BTreeMap;
    use sqlx::{
        Column, Row,
   };

    /********************************************************************************************************************************** */
    /*-- Methode get_data : permet de réaliser un SELECT sur une table de la base de données sans bind de paramètres                    */
    /********************************************************************************************************************************** */
    pub async fn get_data(conn: &sqlx::Pool<sqlx::MySql>, sql_query: &String) -> Result<Vec<String>, sqlx::Error> {
        let rec = sqlx::query(sql_query).fetch_all(conn).await?;
        let rows = rec;
        println!("size rec : {}", rows.len());
        let nb_rows = rows.len();
        let nb_cols = rows[0].columns().len();
        let cols = rows[0].columns();

        println!("\n===> columns : {:?}", cols[0].name());

        let mut cols_names: Vec<String> = Vec::new();
        for i in 0..nb_cols {
            println!("col n°{} : {:?}", i, cols[i].name());
            cols_names.push(cols[i].name().to_string());
        }
        println!("\n===> cols_names : {:?}", cols_names);
       
        // récupérer les données de la base en &str ! utiliser une double boucle for sur les cols et les rows
        let mut response: Vec<String> = Vec::new();
        for r in 0..nb_rows {
            let mut row: String = String::new();
            row.push_str("{");
            for j in 0..nb_cols {
                let col_name = cols_names[j].as_str();
                let val: &str = rows[r].get(col_name);
                if j == (nb_cols - 1) {
                    row.push_str(format!(" {:?} : {:?}", col_name, val).as_str());
                } else {
                    row.push_str(format!(" {:?} : {:?}, ", col_name, val).as_str());
                }
            }
            row.push_str("}");
            response.push(row);
        }
        println!("response: {:?}", &response);
        Ok(response)
    }

    /********************************************************************************************************************************** */
    /*-- Methode set_data : permet de réaliser un INSERT/UPDATE ou DELETE sur une table de la base de données avec plusieurs paramètres */
    /********************************************************************************************************************************** */
    pub async fn set_data(conn: &sqlx::Pool<sqlx::MySql>, mut sql_string:String, keys: Vec<String>) -> u64 {
        println!("\n===> keys.len(): {:?}", keys.len());
        for n in 0..keys.len() {
            sql_string = sql_string.replace(format!(":{}",n).as_str(), &format!("{:?}",keys[n]));
        }
        println!("\n===> sql_string: {:?}", sql_string);
        let stmt = sqlx::query(&sql_string);
        let res = stmt.execute(conn).await.unwrap();
        res.rows_affected()
    }
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