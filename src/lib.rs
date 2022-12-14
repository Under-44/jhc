//-----> global constants

pub const HTML_DEFAULT: &str = "<!DOCTYPE html>\n    <html lang=\"en\">\n    \n    <head>\n      <meta charset=\"UTF-8\">\n      <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n      <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n      <link rel=\"stylesheet\" href=\"./styles/style.css\">\n      <title>Document</title>\n    </head>\n    \n    <body>\n    \n      <script src=\"./javascript/index.js\"></script>\n    </body>\n    \n    </html>\n    ";
pub const JS_DEFAULT: &str = "// \u{2193} do stuff \u{2193}\n    ";
pub const CSS_DEFAULT: &str = "/* \u{2193} do stuff \u{2193} */\n    ";

//-----> functions

/**
 ## path to a file with optional content

 content takes a `Option<&[u8]>` aka bytes\
 pass `None` for no file contents

 ---
 ### example
 ```
//          ↓ path             ↓ content
create_path("folder/file.txt", Some(b"Hello, world!"));
 ```

*/
pub fn create_path(path: &str, content: Option<&[u8]>) -> Result<(), std::io::Error>
{
    //  ↓ split so we can pop last for creating folders
    let file_path = path.split("/").collect::<Vec<&str>>();
    //  ↓ get file name at end of path for bellow
    let file = file_path[file_path.len() - 1];
    //  ↓ remove the end file
    let folder = path.to_owned().replace(&file, "");

    // ↓ create folders
    std::fs::DirBuilder::new().recursive(true).create(&folder)?;

    // ↓ create file
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

    
    use std::io::Write;
    //                     ↓ if none default to no write
    file.write_all(content.unwrap_or(&[0]))?;

    Ok(())
}
