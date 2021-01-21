/*
 * Mirage: A Tiny Markdown Compiler written in Rust
 * Author: Anirudh Rowjee
 */
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    //
    // ptag : test if we are in paragraph
    let mut ptag: bool = false;
    // htag: check if we are in heading
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
        println!("{}", &line_contents);
    }

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
