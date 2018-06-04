pub fn clean_string(string: &str) -> String{
    string.clone().replace("\"", "")
}