use std::fmt::Display;

use json::JsonValue;

#[derive(Clone)]
pub struct Destination {
    id: String,
    pub name: String,
    pub position: (f64, f64),
    description: String,
    nsr_code: String,
}

impl Destination {
    pub fn from_json(object: JsonValue) -> Self {
        let mut id: String = Default::default();
        let mut name: String = Default::default();
        let mut position: (f64, f64) = (0.0, 0.0);
        let mut description: String = Default::default();
        let mut nsr_code: String = Default::default();

        object.entries().for_each(|(k, v)| match k {
            "id" => id = v.to_string(),
            "name" => name = v.to_string(),
            "position" => position = parse_position(v),
            "shortDescription" => description = v.to_string(),
            "externalReferences" => v.entries().for_each(|(_, v)| {
                if v.contains("NSR:") {
                    nsr_code = v.to_string();
                }
            }),
            _ => println!("invalid key: {}", k),
        });

        Self {
            id,
            name,
            position,
            description,
            nsr_code,
        }
    }
}

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, name: {}, position: ({},{}), description: {}",
            self.id, self.name, self.position.0, self.position.1, self.description
        )?;
        Ok(())
    }
}

fn parse_position(object: &JsonValue) -> (f64, f64) {
    let mut lat = 0.0;
    let mut long = 0.0;
    object.entries().for_each(|(k, v)| match k {
        "latitude" => lat = v.as_f64().unwrap(),
        "longitude" => long = v.as_f64().unwrap(),
        _ => panic!("invalid key: {k}"),
    });
    (lat, long)
}
