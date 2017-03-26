//! TODO

use error::*;
use json::JsonValue;
use hupa::Hupa;

/// Convert hupa to json
impl Into<JsonValue> for Hupa {
    fn into(self) -> JsonValue {
        object! {
            "category" => self.get_category(),
            "subCategory" => self.get_sub_category(),
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
        let sub_category = member["subCategory"].as_str().unwrap();
        let origin = member["origin"].as_str().unwrap();
        hupas.push(Hupa::new(category, sub_category, origin));
    }
    Ok(hupas)
}


#[cfg(test)]
mod unit_tests {
    use json;
    use hupa::Hupa;
    use super::*;

    fn vec_of_hupas() -> Vec<Hupa> {
        vec![("test1", "test2", "/"),
             ("os", "gentoo", "/etc/portage"),
             ("dotfiles", "all", "/dotfiles")]
                .into_iter()
                .map(|(c, s, p)| Hupa::new(c, s, p))
                .collect()
    }

    fn stringify_hupa(hupa: &Hupa) -> String {
        format!("{{\"category\":\"{}\",\"subCategory\":\"{}\",\"origin\":\"{}\"}}",
                hupa.get_category(),
                hupa.get_sub_category(),
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
        let json = "[{\"category\":\"test1\",\"subCategory\":\"test2\",\"origin\":\"/\"},{\"category\":\"os\",\"subCategory\":\"gentoo\",\"origin\":\"/etc/portage\"},{\"category\":\"dotfiles\",\"subCategory\":\"all\",\"origin\":\"/dotfiles\"}]";
        let hupas = json_to_hupas(::json::parse(json).unwrap()).unwrap();
        assert_eq!(hupas, vec_of_hupas());
    }
}
