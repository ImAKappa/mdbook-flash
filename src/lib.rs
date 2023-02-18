use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;

/// A no-op preprocessor.
pub struct Flash;

impl Flash {
    pub fn new() -> Flash {
        Flash
    }
}

impl Preprocessor for Flash {
    fn name(&self) -> &str {
        "flashcard-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // TODO: Support `{ }` in flashes and fills; should convert to <span class="back"><\span>
        // TODO: Figure out what `lazy_static!` is, and if it's necessary for regex
        let flash_re = Regex::new(r"\?\?`(?P<c>.+)`")?;
        let replace_flash = "<div class=\"flash\">$c<\\div>";

        let fill_re = Regex::new(r"\?`(?P<c>.+)`")?;
        let replace_fill = "<div class=\"fill\">$c<\\div>";

        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                chapter.content = flash_re.replace_all(&chapter.content, replace_flash).to_string();
                chapter.content = fill_re.replace_all(&chapter.content, replace_fill).to_string();
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mdbook::book::BookItem;

    fn simple_flash_json() -> String {
        let input_json = r##"[
            {
                "root": "/path/to/book",
                "config": {
                    "book": {
                        "authors": ["AUTHOR"],
                        "language": "en",
                        "multilingual": false,
                        "src": "src",
                        "title": "TITLE"
                    },
                    "preprocessor": {
                        "nop": {}
                    }
                },
                "renderer": "html",
                "mdbook_version": "0.4.21"
            },
            {
                "sections": [
                    {
                        "Chapter": {
                            "name": "Chapter 1",
                            "content": "# Chapter 1\n??`Hello there!`",
                            "number": [1],
                            "sub_items": [],
                            "path": "chapter_1.md",
                            "source_path": "chapter_1.md",
                            "parent_names": []
                        }
                    }
                ],
                "__non_exhaustive": null
            }
        ]"##;
        String::from(input_json)
    }

    #[test]
    fn test_flash_run() {

        let input_json = simple_flash_json();
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let result = Flash::new().run(&ctx, book);
        assert!(result.is_ok());
    }

    #[test]
    fn test_flash_html() {
        // Test that flashes are converted to correct html
        let input_json = simple_flash_json();
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let result = Flash::new().run(&ctx, book);

        // Change flash to html
        let actual_content = match &result.unwrap().sections[0] {
            BookItem::Chapter(chapter) => {
                chapter.content.to_string()
            }
            _ => {
                panic!()
            }
        };
        let expected_content = String::from("# Chapter 1\n<div class=\"flash\">Hello there!<\\div>");
        assert_eq!(actual_content, expected_content);
    }

    fn simple_fill_json() -> String {
        let input_json = r##"[
            {
                "root": "/path/to/book",
                "config": {
                    "book": {
                        "authors": ["AUTHOR"],
                        "language": "en",
                        "multilingual": false,
                        "src": "src",
                        "title": "TITLE"
                    },
                    "preprocessor": {
                        "nop": {}
                    }
                },
                "renderer": "html",
                "mdbook_version": "0.4.21"
            },
            {
                "sections": [
                    {
                        "Chapter": {
                            "name": "Chapter 1",
                            "content": "# Chapter 1\n?`Hello there!`",
                            "number": [1],
                            "sub_items": [],
                            "path": "chapter_1.md",
                            "source_path": "chapter_1.md",
                            "parent_names": []
                        }
                    }
                ],
                "__non_exhaustive": null
            }
        ]"##;
        String::from(input_json)
    }

    #[test]
    fn test_fill_html() {
        // Test that blank fills are converted to correct html
                // Test that flashes are converted to correct html
                let input_json = simple_fill_json();
                let input_json = input_json.as_bytes();
        
                let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
                let result = Flash::new().run(&ctx, book);
        
                // Change flash to html
                let actual_content = match &result.unwrap().sections[0] {
                    BookItem::Chapter(chapter) => {
                        chapter.content.to_string()
                    }
                    _ => {
                        panic!()
                    }
                };
                let expected_content = String::from("# Chapter 1\n<div class=\"fill\">Hello there!<\\div>");
                assert_eq!(actual_content, expected_content);
    }
}