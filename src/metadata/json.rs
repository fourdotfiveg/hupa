//! TODO

use json::JsonValue;
use hupa::Hupa;

impl Into<JsonValue> for Hupa {
    fn into(self) -> JsonValue {
        object! {
            "category" => self.get_category(),
            "subCategory" => self.get_sub_category(),
            "origin" => self.get_origin().display().to_string()
        }
    }
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
        let json = json::from(vec_of_hupas());
        let mut output = String::new();
        output.push('[');
        for hupa in vec_of_hupas() {
            output.push_str(&stringify_hupa(&hupa));
            output.push(',');
        }
        output.pop();
        output.push(']');
    }
}
