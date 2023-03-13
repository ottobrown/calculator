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

        let mut functions = HashMap::new();

        fn f_ln(x: f64) -> f64 {
            x.ln()
        }
        let ln: fn(f64) -> f64 = f_ln;
        functions.insert("ln".to_string(), ln);

        fn f_sin(x: f64) -> f64 {
            x.sin()
        }
        let sin: fn(f64) -> f64 = f_sin;
        functions.insert("sin".to_string(), sin);

        fn f_cos(x: f64) -> f64 {
            x.cos()
        }
        let cos: fn(f64) -> f64 = f_cos;
        functions.insert("cos".to_string(), cos);

        fn f_tan(x: f64) -> f64 {
            x.tan()
        }
        let tan: fn(f64) -> f64 = f_tan;
        functions.insert("tan".to_string(), tan);

        Self {
            constants,
            functions,
        }
    }
}
