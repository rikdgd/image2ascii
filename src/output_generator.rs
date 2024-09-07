pub trait OutputGenerator {
    fn from_ascii_image(image: String) -> Self;
    fn generate_output(&self) -> std::io::Result<()>;
}

pub struct HtmlGenerator {
    image: String,
}
impl OutputGenerator for HtmlGenerator {
    fn from_ascii_image(image: String) -> Self {
        Self {
            image
        }
    }

    fn generate_output(&self) -> std::io::Result<()> {
        let html_image_string = get_html_image_string(&self.image);
        // TODO: Write string to ASCII-image.html
        Ok(())
    }
}

fn get_html_image_string(image: &str) -> String {
    let char_iter = image.chars();
    let row_divider = "</p>\n<p>";
    let mut html_image_string = format!("{}<p>", BASE_HTML_START);

    for next_char in char_iter {
        html_image_string.push(next_char);

        if next_char == '\n' {
            html_image_string.push_str(row_divider);
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
            width: 100vw;
            height: 100vh;
            display: flex;
            color: #eaede8;
            font-family: "Monaco", monospace;
            line-height: 120%;
            white-space: pre;
            background-color: rgb(31, 31, 32);
            overflow: auto;
        }
    </style>
</head>
<body>
    "#;
const BASE_HTML_END: &str = r#"
</body>
</html>"#;
