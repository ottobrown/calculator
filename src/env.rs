use std::collections::HashMap;

pub struct Env {
    /// Constants that can be accessed by the parser
    ///
    /// for example, if the map contains `"pi": 3.1415`,
    /// the parser will parse `pi` as `Token::Number(3.1415)`
    ///
    /// strings must consist of ASCII alphabetic chars or '_'
    pub constants: HashMap<String, f64>,
}
impl Default for Env {
    fn default() -> Self {
        let mut constants = HashMap::new();

        constants.insert("pi".to_string(), std::f64::consts::PI);
        constants.insert("e".to_string(), std::f64::consts::E);

        Self { constants }
    }
}
