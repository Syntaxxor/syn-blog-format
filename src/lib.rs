use std::{fs::File, io::{BufRead, BufReader, Write}, path::Path};
use chrono::{DateTime, Local, TimeZone};


// Types

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SynElement {
    Text(String),
    Code(String),
    Heading(String),
    Image{path: String, alt: String, style: String},
    LineH,
}

pub struct SynFile {
    title: String,
    tags: Vec<String>,
    posted: u64,
    summary: String,
    elements: Vec<SynElement>,
}


// Implementations

impl SynElement {
    fn parse_line(line: String) -> Result<Self, ()> {
        let line = line.trim().to_string();
        if line == "---" {
            Ok(SynElement::LineH)
        } else if line.starts_with("#") {
            Ok(SynElement::Heading(line[1..].to_string()))
        } else if line.starts_with(".img ") {
            let sections = line[5..].split("|").map(|e| e.to_string()).collect::<Vec<_>>();
            if sections.len() == 3 {
                Ok(SynElement::Image { path: sections[0].clone(), alt: sections[1].clone(), style: sections[2].clone() })
            } else {
                Err(())
            }
        } else if line.starts_with(".code ") {
            Ok(SynElement::Code(line[6..].to_string()))
        } else {
            Ok(SynElement::Text(line))
        }
    }


    pub fn generate_tag(&self) -> String {
        match self {
            SynElement::Text(text) => {
                let text = text.replace("\n", "<br>");
                format!("<p>{text}</p>")
            },
            SynElement::Code(text) => {
                format!("<p class='code'>{text}</p>")
            },
            SynElement::Heading(text) => format!("<h2>{text}</h2>"),
            SynElement::Image { path, alt, style } => format!("<img src='{path}' style='{style}'>{alt}</img>"),
            SynElement::LineH => "<div class='hline'></div>".into(),
        }
    }


    fn generate_line(&self) -> String {
        match self {
            SynElement::Text(text) => text.clone(),
            SynElement::Code(text) => format!(".code {text}"),
            SynElement::Heading(text) => format!("#{text}"),
            SynElement::Image { path, alt, style } => format!(".img {path}|{alt}|{style}"),
            SynElement::LineH => "---".into(),
        }
    }
}


impl SynFile {
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        let in_file = File::open(path);
        if let Ok(in_file) = in_file {
            let mut reader = BufReader::new(in_file);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let title = line.trim().to_string();
            line.clear();
            reader.read_line(&mut line).unwrap();
            let tags = line.split(",").map(|e| e.trim().to_string()).collect::<Vec<_>>();
            line.clear();
            reader.read_line(&mut line).unwrap();
            let posted = line.trim().parse::<u64>().unwrap_or(0);
            line.clear();
            reader.read_line(&mut line).unwrap();
            let summary = line.trim().to_string();

            let mut elements = Vec::new();
            while let Ok(len) = reader.read_line(&mut line) {
                if len <= 1 {
                    if let Ok(element) = SynElement::parse_line(line.clone()) {
                        elements.push(element);
                    }

                    line.clear();
                }
                if len == 0 {
                    break;
                }
            }


            return Ok(Self {
                title,
                tags,
                posted,
                summary,
                elements,
            });
        }

        Err(())
    }


    pub fn load_file_metadata<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        let in_file = File::open(path);
        if let Ok(in_file) = in_file {
            let mut reader = BufReader::new(in_file);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let title = line.trim().to_string();
            line.clear();
            reader.read_line(&mut line).unwrap();
            let tags = line.split(",").map(|e| e.trim().to_string()).collect::<Vec<_>>();
            line.clear();
            reader.read_line(&mut line).unwrap();
            let posted = line.trim().parse::<u64>().unwrap_or(0);
            line.clear();
            reader.read_line(&mut line).unwrap();
            let summary = line.trim().to_string();

            let elements = Vec::new();

            return Ok(Self {
                title,
                tags,
                posted,
                summary,
                elements,
            });
        }
        Err(())
    }


    pub fn save_file<P: AsRef<Path>>(&self, path: P) {
        let out_file = File::create(path);
        if let Ok(mut out_file) = out_file {
            write!(out_file, "{}\n{}\n{}\n{}\n\n", self.title, self.tags.join(","), self.posted, self.summary).unwrap();
            for element in &self.elements {
                write!(out_file, "{}\n\n", element.generate_line()).unwrap();
            }
        }
    }


    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn get_posted(&self) -> &u64 {
        &self.posted
    }
    pub fn get_posted_str(&self) -> String {
        let date_time = Local.timestamp_opt(*self.get_posted() as i64, 0).unwrap();

        format!("{}", date_time.format("%a, %d %b %Y %H:%M:%S %z"))
    }
    pub fn get_summary(&self) -> &String {
        &self.summary
    }
    pub fn get_elements(&self) -> &Vec<SynElement> {
        &self.elements
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_element() {
        let element = SynElement::parse_line("Hello,\nSynBlog!".into()).unwrap();
        assert_eq!(element, SynElement::Text("Hello,\nSynBlog!".into()));

        let as_line = element.generate_line();
        assert_eq!(as_line, "Hello,\nSynBlog!".to_string());

        let as_tag = element.generate_tag();
        assert_eq!(as_tag, "<p>Hello,<br>SynBlog!</p>");
    }

    #[test]
    fn img_element() {
        let element = SynElement::parse_line(".img test.png|A test image!|width:100%".into()).unwrap();
        assert_eq!(element, SynElement::Image { path: "test.png".into(), alt: "A test image!".into(), style: "width:100%".into() });

        let as_line = element.generate_line();
        assert_eq!(as_line, ".img test.png|A test image!|width:100%".to_string());

        let as_tag = element.generate_tag();
        assert_eq!(as_tag, "<img src='test.png' style='width:100%'>A test image!</img>");
    }

    #[test]
    fn heading_element() {
        let element = SynElement::parse_line("#Big Title".into()).unwrap();
        assert_eq!(element, SynElement::Heading("Big Title".into()));

        let as_line = element.generate_line();
        assert_eq!(as_line, "#Big Title".to_string());

        let as_tag = element.generate_tag();
        assert_eq!(as_tag, "<h2>Big Title</h2>");
    }

    #[test]
    fn lineh_element() {
        let element = SynElement::parse_line("---".into()).unwrap();
        assert_eq!(element, SynElement::LineH);

        let as_line = element.generate_line();
        assert_eq!(as_line, "---".to_string());

        let as_tag = element.generate_tag();
        assert_eq!(as_tag, "<div class='hline'></div>");
    }
}
