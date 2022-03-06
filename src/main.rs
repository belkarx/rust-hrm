use serde::Deserialize;
use std::fs;
use std::collections::BTreeMap as Map;
use std::error::Error;

//The optional values
#[derive(Debug, Default)]
struct Subject {
    source: String,
    name: String, 
}

//The less optional values
#[derive(Debug, Deserialize)]
struct RawSubject {
    alias: String, //required
    uses: Uses, //required
    contact: Map<String, String>, //required
    source: Option<String>,
    name: Option<String>, 
}

//Everything here is required
#[derive(Debug, Deserialize)]
struct Uses {
    form: String,
    details: Vec<String>,
}

fn read_data(input: &str) -> Result<Map<String, Subject>, Box<Error>> {
    let raw_subjects: Vec<RawSubject> = serde_json::from_str(input)?;

    let mut m = Map::new();
    for raw in raw_subjects {
        let map_entry = m.entry(raw.alias).or_insert_with(Subject::default);

        // One push for every vector in the struct, even for missing observations
        map_entry.source.push(raw.luminosity);
        map_entry.contact.push(raw.color);
    }
    Ok(m)
}


fn main() {
let input = r##"[
                      {
                        "sensor": "left",
                        "luminosity": "50",
                        "color": "(255,0,0)"
                      },
                      {
                        "sensor": "left",
                        "color": "#0f0"
                      },
                      {
                        "sensor": "right",
                        "luminosity": "20"
                      },
                      {
                        "sensor": "right",
                        "luminosity": "40",
                        "color": "(255,0,0)"
                      },
                      {
                        "sensor": "left",
                        "luminosity": "30"
                      },
                      {
                        "sensor": "top",
                        "luminosity": "10"
                      },
                      {
                        "sensor": "right",
                        "color": "(0,0,0)"
                      }
                    ]"##;
    let m = read_sensor_data(input).unwrap();
    println!("{:#?}", m);
}




    let file = fs::read_to_string("text.json").expect("Unable to read file");

    let subject: Person = serde_json::from_str(&file).expect("JSON was not well-formatted");
    println!("{:?}", person)
}
/*
enum Form {
    Professional,
    Informative,
    Social
}*/
