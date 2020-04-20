pub fn hello() -> String {
    "world".to_string()
}

/// Convert a `Vec<&str>` to a `Vec<String>`
pub fn to_vec_string(vstr: &Vec<&str>) -> Vec<String> {
    vstr.iter().map(|s| s.to_string()).collect()
}

/// Convert a `Vec<&str>` to a `Vec<String>` where the strings are upper case
pub fn to_vec_uc_string(vstr: &Vec<&str>) -> Vec<String> {
    vstr.iter().map(|s| s.to_string().to_uppercase()).collect()
}

/// Join a `Vec` of `String` into a `String`
pub fn vec_string_to_string(v: &Vec<String>) -> String {
    let r2 = v.iter().fold(String::from(""), |mut sum, the_str| {
        sum.push_str(the_str);
        sum
    });
    r2
}

