//! TODO

use error::*;
use json::JsonValue;
use hupa::Hupa;

/// Convert hupa to json
impl Into<JsonValue> for Hupa {
    fn into(self) -> JsonValue {
        object! {
            "category" => self.get_category(),
            "subCategories" => self.get_sub_categories().clone(),
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
        let category = member["category"].as_str().unwrap();
        let sub_categories_json = &member["subCategories"];
        let mut sub_categories = Vec::new();
        for i in 0..sub_categories_json.len() {
            let sub_category = &sub_categories_json[i];
            sub_categories.push(sub_category.as_str().unwrap().to_owned());
        }
        let origin = member["origin"].as_str().unwrap();
        hupas.push(Hupa::new(category, &sub_categories, origin));
    }
    Ok(hupas)
}


#[cfg(test)]
mod unit_tests {
    use json;
    use hupa::Hupa;
    use super::*;

    fn vec_of_hupas() -> Vec<Hupa> {
        vec![("test1", vec!["test2"], "/"),
             ("os", vec!["gentoo"], "/etc/portage"),
             ("dotfiles", vec!["all"], "/dotfiles")]
                .into_iter()
                .map(|(c, s, p)| Hupa::new(c, &s.iter().map(|s| s.to_string()).collect(), p))
                .collect()
    }

    fn stringify_hupa(hupa: &Hupa) -> String {
        let mut sub_str = String::new();
        sub_str.push('[');
        sub_str.push_str(hupa.get_sub_categories()
                             .iter()
                             .map(|s| format!("\"{}\",", s))
                             .collect::<String>()
                             .as_str());
        sub_str.pop();
        sub_str.push(']');
        format!("{{\"category\":\"{}\",\"subCategories\":{},\"origin\":\"{}\"}}",
                hupa.get_category(),
                sub_str,
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

    #[test]
    fn test_json_to_hupas() {
        let json = "[{\"category\":\"test1\",\"subCategories\":[\"test2\"],\"origin\":\"/\"},{\"category\":\"os\",\"subCategories\":[\"gentoo\"], \"origin\":\"/etc/portage\"},{\"category\":\"dotfiles\",\"subCategories\":[\"all\"],\"origin\":\"/dotfiles\"}]";
        let hupas = json_to_hupas(::json::parse(json).unwrap()).unwrap();
        assert_eq!(hupas, vec_of_hupas());
    }
}
