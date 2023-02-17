pub use super::*;

#[test]
fn nop_preprocessor_run() {
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
                        "content": "# Chapter 1\n",
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
    let input_json = input_json.as_bytes();

    let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
    let expected_book = book.clone();
    let result = Nop::new().run(&ctx, book);
    assert!(result.is_ok());

    // The nop-preprocessor should not have made any changes to the book content.
    let actual_book = result.unwrap();
    assert_eq!(actual_book, expected_book);
}