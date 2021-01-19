/*
 * Mirage: A Tiny Markdown Compiler written in Rust
 * Author: Anirudh Rowjee
 */

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
fn parse_markdown_file()
{
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

fn main()
{
    usage();
}
