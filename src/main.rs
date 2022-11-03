use std::{fs, io::Write};

// bootleg af
fn main() {
    // ↓ ENV OPTIONS ↓
    let mut name_of_project = "";
    let mut is_arg = false;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        is_arg = true;
        name_of_project = &args[1];
    }

    // ↓ GLOBALS ↓
    const HTML_DEFAULT: &str = r#"<!DOCTYPE html>
    <html lang="en">
    
    <head>
      <meta charset="UTF-8">
      <meta http-equiv="X-UA-Compatible" content="IE=edge">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <link rel="stylesheet" href="./styles/style.css">
      <title>Document</title>
    </head>
    
    <body>
    
      <script src="./javascript/index.js"></script>
    </body>
    
    </html>
    "#;

    const JS_DEFAULT: &str = r#"// ↓ do stuff ↓
    "#;

    const CSS_DEFAULT: &str = r#"/* ↓ do stuff ↓ */
    "#;

    // ↓ CREATE FILES ↓

    // with args
    if is_arg {
        fs::create_dir(name_of_project).unwrap();

        fs::create_dir(format!("{name_of_project}/styles")).unwrap();
        fs::create_dir(format!("{name_of_project}/javascript")).unwrap();

        let mut js_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{name_of_project}/javascript/index.js"))
            .unwrap();

        let mut css_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{name_of_project}/styles/style.css"))
            .unwrap();

        let mut html_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{name_of_project}/index.html"))
            .unwrap();

        html_file.write_all(HTML_DEFAULT.as_bytes()).unwrap();
        js_file.write_all(JS_DEFAULT.as_bytes()).unwrap();
        css_file.write_all(CSS_DEFAULT.as_bytes()).unwrap();

        return;
    }

    // without args
    fs::create_dir("styles").unwrap();
    fs::create_dir("javascript").unwrap();

    let mut js_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("javascript/index.js")
        .unwrap();

    let mut css_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("styles/style.css")
        .unwrap();

    let mut html_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("index.html")
        .unwrap();

    // ↓ WRITE FILES ↓
    html_file.write_all(HTML_DEFAULT.as_bytes()).unwrap();
    js_file.write_all(JS_DEFAULT.as_bytes()).unwrap();
    css_file.write_all(CSS_DEFAULT.as_bytes()).unwrap();
}
