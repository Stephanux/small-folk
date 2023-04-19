pub mod sql_request {
    use handlebars::Handlebars;
    use serde_json::{self, Value};
    use std::collections::BTreeMap;


    /* Methode get_data : permet de réaliser un SELECT sur une ou un ensemble de tables */
    pub async fn get_data(conn: &rusqlite::Connection, sql_query: &String) -> rusqlite::Result<Vec<String>> {
        let mut stmt = conn.prepare(sql_query)?;
        // cette façon de copier et récupérer les données du statement évite le MRE sur la variable stmt.
        let typescol: Vec<String> = stmt
            .columns()
            .into_iter()
            .map(|x| x.decl_type().unwrap().clone().to_string())
            .collect();
        let cols_type: Vec<String> = typescol.clone();
        let nbcols: usize = stmt.column_count();
        // cette façon de copier et récupérer les données du statement évite le MRE sur la variable stmt.
        let cols_names: Vec<String> = stmt
            .column_names()
            .into_iter()
            .map(|x| x.to_owned())
            .collect();
        let mut data = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            let mut json_data = String::new();
            json_data.push_str(&"{".to_owned());
            let mut val_str = String::new();
            for i in 0..nbcols {
                if cols_type[i].contains("VARCHAR") {
                    let val: rusqlite::Result<Option<String>> = row.get(i);
                    val_str = match val {
                        Ok(Some(val)) => format!("\"{}\": {:?}", cols_names[i], val),
                        Ok(None) => format!("\"{}\": \"null\"", cols_names[i]),
                        Err(e) => return Err(e),
                    };
                } else if cols_type[i].contains(&"NUMERIC".to_string())
                    && cols_type[i].contains(&','.to_string())
                {
                    let val: rusqlite::Result<Option<f32>> = row.get(i);
                    val_str = match val {
                        Ok(Some(val)) => format!("\"{}\": {:?}", cols_names[i], val),
                        Ok(None) => format!("\"{}\": \"null\"", cols_names[i]),
                        Err(e) => return Err(e),
                    };
                } else if cols_type[i].contains(&"NUMERIC".to_string())
                    && !cols_type[i].contains(&",".to_string())
                {
                    let val: rusqlite::Result<Option<u64>> = row.get(i);
                    val_str = match val {
                        Ok(Some(val)) => format!("\"{}\": {:?}", cols_names[i], val),
                        Ok(None) => format!("\"{}\": \"null\"", cols_names[i]),
                        Err(e) => return Err(e),
                    };
                }
                json_data.push_str(&val_str);
                if i < nbcols - 1 {
                    json_data.push_str(&", ".to_owned());
                }
            }
            json_data.push_str(&"}".to_owned());
            data.push(json_data);
        }
        Ok(data) // return Vec<String> contenant les lignes de la requête sous un format pseudo JSON exploitable par serde_json
    }

    /* Methode set_data : permet de réaliser un INSERT/UPDATE ou DELETE sur une table de la base de données avec plusieurs paramètres */
    pub fn set_data(conn: &rusqlite::Connection, sql_query: &String, params: Vec<String>) -> usize {
        let mut stmt = conn.prepare(sql_query).unwrap();
        let nbrows = params.len();
        for i in 0..nbrows {
            let result = stmt.raw_bind_parameter(i + 1, &params[i]);
            println!("result : {:?}", result);
        }
        let result = stmt.raw_execute();
        println!("result : {:?}", result);
        result.unwrap()
    }

    pub fn json_to_html(data: Vec<Value>) -> Result<String, handlebars::RenderError> {
        let mut handlebars = Handlebars::new();
        let _resp = handlebars
            .register_template_file("datahbs", "./tableau_html.hbs")
            .unwrap();
        // ici gerer le retour du parsing hbs via "_resp" et tester si error... match _resp { ...un truc du genre
        let mut datahbs: BTreeMap<String, Vec<Value>> = BTreeMap::new();
        datahbs.insert("data".to_string(), data);
        println!(
            "\n==> handlebars : {:?}",
            handlebars.render_template("datahbs", &datahbs)
        );
        handlebars.render("datahbs", &datahbs)
    }
}
