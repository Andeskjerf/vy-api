use json::JsonValue;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Duration {
    days: u8,
    hours: u8,
    minutes: u8,
}

impl Duration {
    pub fn new(days: u8, hours: u8, minutes: u8) -> Self {
        Self {
            days,
            hours,
            minutes,
        }
    }

    pub fn from_json(object: JsonValue) -> Self {
        let mut days = 0;
        let mut hours = 0;
        let mut minutes = 0;
        object.entries().for_each(|(k, v)| match k {
            "days" => days = v.as_u8().unwrap(),
            "hours" => hours = v.as_u8().unwrap(),
            "minutes" => minutes = v.as_u8().unwrap(),
            _ => println!("invalid key: {k:?}"),
        });
        Self {
            days,
            hours,
            minutes,
        }
    }
}
