use std::collections::HashMap;

pub struct Env {
    /// Constants that can be accessed by the parser
    ///
    /// for example, if the map contains `"pi": 3.1415`,
    /// the parser will parse `pi` as `Token::Number(3.1415)`
    ///
    /// strings must consist of ASCII alphabetic chars or '_'
    pub constants: HashMap<String, f64>,

    pub functions: HashMap<String, fn(f64) -> f64>,
}
impl Default for Env {
    fn default() -> Self {
        let mut constants = HashMap::new();
        constants.insert("pi".to_string(), std::f64::consts::PI);
        constants.insert("e".to_string(), std::f64::consts::E);
        constants.insert("degrees".to_string(), std::f64::consts::PI / 180.0);
        constants.insert("deg".to_string(), std::f64::consts::PI / 180.0);

        let mut functions = HashMap::new();

        make_fn(&mut functions, "ln", |x| x.ln());
        make_fn(&mut functions, "sin", |x| x.sin());
        make_fn(&mut functions, "cos", |x| x.cos());
        make_fn(&mut functions, "tan", |x| x.tan());
        make_fn(&mut functions, "sqrt", |x| x.sqrt());
        make_fn(&mut functions, "cbrt", |x| x.cbrt());
        make_fn(&mut functions, "acos", |x| x.acos());
        make_fn(&mut functions, "asin", |x| x.asin());
        make_fn(&mut functions, "atan", |x| x.atan());

        Self {
            constants,
            functions,
        }
    }
}

fn make_fn(map: &mut HashMap<String, fn(f64) -> f64>, name: &str, f: fn(f64) -> f64) {
    let f_f: fn(f64) -> f64 = f;

    map.insert(name.to_string(), f_f);
}
