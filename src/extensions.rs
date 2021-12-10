pub struct Extensions;

impl Extensions {
    pub fn get() -> [&'static str; 2] {
        ["txt", "srt"]
    }
}

impl std::fmt::Display for Extensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = Extensions::get().iter().fold(String::new(), |a, b| (a.to_string() + " ." + b));
        f.write_str(&data)
    }
}