use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    pub personality: HashMap<String, Personality>,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Personality {
    prompt: String,
    hobbies: Option<Vec<String>>,
}

impl Personality {
    pub fn summary(&self) -> String {
        let mut summary = String::new();

        summary.push_str(&self.prompt);

        if let Some(hobbies) = &self.hobbies {
            match hobbies.len() {
                0 => (),
                1 => summary.push_str(&format!(" Your hobbie is {}.", hobbies[0])),
                _ => {
                    let mut hobbie_string = String::new();
                    hobbie_string.push_str("Your hobbies are ");
                    for hobbie in &hobbies[..hobbies.len() - 2] {
                        hobbie_string.push_str(&format!("{hobbie}, "));
                    }
                    let [.., second_last, last] = hobbies.as_slice() else {
                        unreachable!(
                            "If you see this in production, you are the God's choosen one, go and make that 4th temple."
                        )
                    };
                    hobbie_string.push_str(&format!("{second_last} and {last}"));

                    summary.push_str(&format!(" {hobbie_string}."));
                }
            }
        }

        summary
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn personality_summaring_single_hobbie() {
        let personlaity_text =
            "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
                .to_string();

        let personality = Personality {
            prompt: personlaity_text,
            hobbies: Some(vec!["coding".into()]),
        };

        assert_eq!(personality.summary(), "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades. Your hobbie is coding.".to_string());
    }

    #[test]
    fn personality_summaring_multiple_hobbies() {
        let personlaity_text =
            "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
                .to_string();

        let personality = Personality {
            prompt: personlaity_text,
            hobbies: Some(vec!["anime".into(), "coding".into(), "hacking".into()]),
        };

        assert_eq!(personality.summary(), "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades. Your hobbies are anime, coding and hacking.".to_string());
    }

    #[test]
    fn personality_summaring_no_hobbies() {
        let personlaity_text =
            "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
                .to_string();

        let personality = Personality {
            prompt: personlaity_text,
            hobbies: None,
        };

        assert_eq!(
            personality.summary(),
            "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
                .to_string()
        );
    }

    #[test]
    fn deserialization() {
        let config = Config {
            personality: {
                let mut hash_map = HashMap::new();
                hash_map.insert("kuro".into(), Personality { prompt: "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades.".into(), hobbies: Some(vec!["hacking".into(), "coding".into()]) });
                hash_map
            },
        };

        let toml_text = r#"
[personality.kuro]
prompt = "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
hobbies = [ "hacking", "coding" ]
        "#;

        let tomul = toml::from_str::<Config>(toml_text).unwrap();

        assert_eq!(tomul, config)
    }
}
