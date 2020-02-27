#[derive(Debug)]
pub struct Package<'a> {
    pub options: Vec<&'a str>,
    pub package_str: String,
}

impl <'a> Package<'a> {
    pub fn get_code(&self) -> String {
        format!("\\usepackage{}{{{}}}", &self.get_options(), &self.package_str)
    }

    fn get_options(&self) -> String {
        match self.options.len() {
            0 => "".to_string(),
            _ => format!("[{}]", &self.options.join(","))
        }
    }
}

#[test]
fn basic_test () {
    let pack = Package {
        package_str: "Test package string".to_string(),
        options: vec!["foo", "bar"]
    };
    assert_eq!(&pack.get_code(),
        "\\usepackage[foo,bar]{Test package string}"
    );
}

