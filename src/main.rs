fn main() {
    // ↓ create files
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        jhc::create_path(&format!("{}/index.html", &args[1]), Some(jhc::HTML_DEFAULT.as_bytes())).unwrap();
        jhc::create_path(&format!("{}/styles/style.css", &args[1]), Some(jhc::CSS_DEFAULT.as_bytes())).unwrap();
        jhc::create_path(&format!("{}/javascript/index.js", &args[1]), Some(jhc::JS_DEFAULT.as_bytes())).unwrap();
        return;
    }
    jhc::create_path("index.html", Some(jhc::HTML_DEFAULT.as_bytes())).unwrap();
    jhc::create_path("styles/style.css", Some(jhc::CSS_DEFAULT.as_bytes())).unwrap();
    jhc::create_path("javascript/index.js", Some(jhc::JS_DEFAULT.as_bytes())).unwrap();
}
