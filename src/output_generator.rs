use std::fs::OpenOptions;
use std::io::Write;

pub trait OutputGenerator {
    fn from_ascii_image(image: String) -> Self;
    fn generate_output(&self) -> std::io::Result<()>;
}

pub struct HtmlGenerator {
    image: String,
    output_path: String,
}
impl OutputGenerator for HtmlGenerator {
    fn from_ascii_image(image: String) -> Self {
        Self {
            image,
            output_path: String::from("./ascii-image.html"),
        }
    }

    fn generate_output(&self) -> std::io::Result<()> {
        let html_image_string = get_html_image_string(&self.image);
        let mut options = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.output_path)?;
        
        options.set_len(0)?;
        options.write_all(html_image_string.as_bytes())?;
        Ok(())
    }
}

impl HtmlGenerator {
    #[allow(unused)]
    pub fn set_output_path<S: Into<String>>(&mut self, path: S) {
        self.output_path = path.into();
    }
}

fn get_html_image_string(image: &str) -> String {
    let char_iter = image.chars();
    let row_divider = "</p>\n       <p>";
    let mut html_image_string = format!("{}<p>", BASE_HTML_START);

    for next_char in char_iter {
        if next_char == '\n' {
            html_image_string.push_str(row_divider);
        } else {
            html_image_string.push(next_char);
        }
    }

    html_image_string.push_str("</p>");
    html_image_string.push_str(BASE_HTML_END);
    
    html_image_string
}

const BASE_HTML_START: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ASCII image</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        html, body {
            height: 100%;
        }

        p {
            color: #eaede8;
            font-size: 2px;
            font-family: "Monaco", monospace;
        }
        
        .image-container {
            width: 100vw;
            height: 100vh;
            color: #eaede8;
            background-color: rgb(31, 31, 32);
        }
    </style>
</head>
<body>
    <div class="image-container">
        "#;
const BASE_HTML_END: &str = r#"
    </div>
</body>
</html>"#;
