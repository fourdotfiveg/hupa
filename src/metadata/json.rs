//! TODO

use error::*;
use json::JsonValue;
use hupa::Hupa;

/// Convert hupa to json
impl Into<JsonValue> for Hupa {
    fn into(self) -> JsonValue {
        object! {
            "name" => self.get_name(),
            "desc" => self.get_desc(),
            "categories" => self.get_categories().clone(),
            "origin" => self.get_origin().display().to_string()
        }
    }
}

/// Convert json to hupas
pub fn json_to_hupas(json: JsonValue) -> Result<Vec<Hupa>> {
    let mut hupas = Vec::new();
    if !json.is_array() {
        // TODO error
        return Ok(hupas);
    }
    for member in json.members() {
        let name = member["name"].as_str().unwrap();
        let desc = member["desc"].as_str().unwrap();
        let categories_json = &member["categories"];
        let mut categories = Vec::new();
        for i in 0..categories_json.len() {
            let category = &categories_json[i];
            categories.push(category.as_str().unwrap().to_owned());
        }
        let origin = member["origin"].as_str().unwrap();
        hupas.push(Hupa::new(name, desc, categories, origin));
    }
    Ok(hupas)
}


#[cfg(test)]
mod unit_tests {
    use json;
    use hupa::Hupa;

    fn vec_of_hupas() -> Vec<Hupa> {
        vec![("test1", vec!["test2"], "/"),
             ("os", vec!["gentoo"], "/etc/portage"),
             ("dotfiles", vec!["all"], "/dotfiles")]
                .into_iter()
                .map(|(n, c, p)| Hupa::new(n, "", c.iter().map(|s| s.to_string()).collect(), p))
                .collect()
    }

    fn stringify_hupa(hupa: &Hupa) -> String {
        let mut cat_str = String::new();
        cat_str.push('[');
        cat_str.push_str(hupa.get_categories()
                             .iter()
                             .map(|s| format!("\"{}\",", s))
                             .collect::<String>()
                             .as_str());
        cat_str.pop();
        cat_str.push(']');
        format!("{{\"name\":\"{}\",\"desc\":\"{}\",\"categories\":{},\"origin\":\"{}\"}}",
                hupa.get_name(),
                hupa.get_desc(),
                cat_str,
                hupa.get_origin().display())

    }

    #[test]
    fn test_hupa_to_json() {
        for hupa in vec_of_hupas() {
            let hupa_clone = hupa.clone();
            let json = json::from(hupa_clone);
            assert_eq!(json.dump(), stringify_hupa(&hupa));
        }
    }

    #[test]
    fn test_hupas_to_json() {
        let json = json::stringify(vec_of_hupas());
        println!("{}", json);
        let mut output = String::new();
        output.push('[');
        for hupa in vec_of_hupas() {
            output.push_str(&stringify_hupa(&hupa));
            output.push(',');
        }
        output.pop();
        output.push(']');
        assert_eq!(json, output);
    }
}
