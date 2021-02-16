# Mirage - A tiny Markdown Rendering Engine

Inspired by the [tutorial](https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/)

## Usage

1. Clone this Repo.
2. Execute the following 
```console 
$ cargo run <markdown-file-name>
```
3. The HTML should be written to a file (`<markdown-file-name>.html`)

## Status
Currently supports only top-level headings and plain text, such as

```markdown
# This is a top-level heading
This is some text below it.
# This is another top level-heading
This is some more text.
```

## Roadmap
* [ ] Add support for multi-level headings
* [ ] Use a better way to keep track of what is being parsed (perhaps use a stack to keep track?)
* [ ] Recurive Parsing
* [ ] More Error Checking


Feel Free to fork the repo and contribute!





