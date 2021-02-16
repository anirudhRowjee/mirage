/*
 * Mirage: A Tiny Markdown Compiler written in Rust
 * Author: Anirudh Rowjee
 */
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// Function to get the title of the application
fn get_title() -> String
{
    // mutable so that we can append all the other bits of the banner on this
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    // we aren't operating on this, so it's immutable
    let version  = String::from(env!("CARGO_PKG_VERSION"));
    let description = String::from(env!("CARGO_PKG_DESCRIPTION"));
    title.push_str(" (v");
    // since we cannot push an object, we use a slice of the string to append to this
    title.push_str(&version);
    title.push_str("), ");
    title.push_str(&description);
    // return the title
    title
}


/// Function to parse the markdown file - the actual compilation
/// How it should work in essence - 
/// 1. Accept filepath as argument (TODO implement better error checking)
/// 2. Open the file, parse contents into a buffer
/// 3. Convert buffer to HTML and write resultant HTML to a new file
/// If a filepath is not provided, show the usage
///
/// Arguments: provide filename
fn parse_markdown_file(filename: &str)
{
    println!("[INFO] Beginning compilation of file {}", filename);

    // declare filepath as a std::path::Path object
    // it's literally a Path variable
    let inputfile = Path::new(filename);

    // attempt to open the file
    // chain .expect() to handle exceptions for us
    let file = File::open(inputfile)
                .expect("{ ERROR } File opening failed!");

    // state variables - we use this to track the parsing of the line. We use this to tell us
    // what part of structure of the markdown/html conversion we are in, and what we should do when
    // we encounter other closing tags, or, say, a newline character.
    // TODO think of a better way to do this - use a stack?

    // ptag : test if we are in paragraph
    // htag: check if we are in heading
    let mut ptag: bool = false;
    let mut htag: bool = false;

    // instantiate a vector to store the resultant HTML before we write it to a file.
    // A token is essentially the smallest lexical unit of a file, and we use this vector to store
    // the tokens we have in the file post-compile.
    let mut tokens: Vec<String> = Vec::new();

    // instantiate the buffered reader, which helps us read the file into memory.
    // acts as a window into the file - heavy lifting done behind the scenes
    let reader = BufReader::new(file);

    // parse the file line by line
    for line in reader.lines()
    {
        // since line is also a Result object, we have to be careful about how we handle it
        // we use "unwrap" to do this - verbose method of checking for error, and not caring if
        // it's a garbage value. Alternatively you can do
        // ```
        // let line_contents = match line {
        //  Ok(contents) => contents,
        //  Err(e) => panic!("compiler broke, sorry")
        // }
        // ```
        let line_contents = line.unwrap();
        // println!("{}", &line_contents);

        let mut output_line = String::new();
        // we now have to parse the line character by character.
        // we start by getting the first character
        let mut first_char: Vec<char>  = line_contents.chars().take(1).collect();

        // start deciding things
        // since we only support H1 and P tags as of now, this will suffice
        match first_char.pop()
        {
            Some('#') => {
                // if it's open, close it - same for both tags
                if ptag {
                    ptag = false;
                    output_line.push_str("</p>\n");
                }
                if htag {
                    htag = false;
                    output_line.push_str("</h1>\n");
                }

                // add a new heading, formally
                htag = true;
                output_line.push_str("<h1>");

                // push every character apart from the hashtag onto the new string
                // depends on the spec I guess, but in essence it should be one space between the
                // hashtag and the start of the heading
                output_line.push_str(&line_contents[1..]);
            }
            ,
            _ => {
                if !ptag {
                    ptag = true;
                    output_line.push_str("<p>");
                }
                output_line.push_str(&line_contents);
            }
        }

        // even though most browsers would handle this, we need to cover all tags properly.
        // check if there is an open paragraph tag 
        if ptag {
            ptag = false;
            output_line.push_str("</p>\n");
        }
        // check if there is an open heading tag
        if htag {
            htag = false;
            output_line.push_str("</h1>\n");
        }

        // if the line was empty, it would be considered a blank p tag
        // making sure we don't push blank paragraph tags, we just push the contents of this line
        // to the output string (tokens)
        if output_line != "<p></p>\n" 
        {
            tokens.push(output_line);
        }
    }

    /*
    println!("DEBUG >>> ");
    for line in &tokens {
        println!("{}", line);
    }
    */


    // derive the name of the HTML file from the markdown file
    let filename_length = filename.len() - 3;
    let mut new_filename = String::new();
    // make a new string with the name of the file, plus ".html"
    new_filename.push_str(&filename[..filename_length]);
    new_filename.push_str(".html");

    // open the new HTML file for writing 
    let mut output_file = File::create(&new_filename).expect(" { ERROR } Could not open file to write! ");

    for line in &tokens {
        output_file.write_all(line.as_bytes()).expect(" { ERROR } Could not write to file! ")
    }

    println!("[INFO] Parsing complete! Output written to {}", &new_filename);

}


/// Print the short banner, to be run on every command regardless 
fn shortbanner()
{
    let banner_title = get_title();
    println!("{}", banner_title);
}


/// Print the long banner, preferably on wrong usage or `--help`
fn longbanner()
{
    shortbanner();
    // now we create a new mutable string to stack all out instructions onto
    // we have to print the Author's name, their homepage and the Usage Instructions
    let mut author_string = String::from("Written by: ");
    let mut homepage_string = String::from("Homepage: ");
    let usage_string = "Usage: mirage <somefile>.md";

    // mutate
    author_string.push_str(env!("CARGO_PKG_AUTHORS"));
    homepage_string.push_str(env!("CARGO_PKG_HOMEPAGE"));

    // print
    println!("{}\n{}\n{}", author_string, homepage_string,  usage_string);
}

/// Consolidated Banner Printer / general usage guidelines
fn usage()
{
    // shortbanner();
    longbanner();
}

/// Usage
/// `$ mirage ` - prints out the banner
/// `$ mirage <filename.md>` - runs the compiler on the contents of `filename.md`
/// `$ mirage opt1 opt2` - prints usage instructions as we accept only one argument
///
/// NOTE: As of now, the compiler does not support full commonmark standards. It will only
/// parse h1 and p tags, more features to be implemented.
///
fn main()
{
    // parse command line arguments, use a vector of strings to do this
    // internal note: we use `.collect()` to convert our iterator of args into a collection
    let cl_args: Vec<String> = std::env::args().collect();

    // check for number of arguments using match block
    match cl_args.len() {
        // since we have only the filename and the program name, we can use this
        2 => parse_markdown_file(&cl_args[1]),
        // the underscore represents the default case - if nothing else is matched, the 
        // function attached to underscore will handle the expression
        _ => {
            // instead of single function calls, we can have blocks too
            println!("ERROR! This is not the way.");
            usage();
        }
    }

}
