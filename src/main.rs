
fn get_version() -> u16
{
    1000
}

fn usage()
{

    // test variable declaration with automatic type inference
    let the_version = get_version();

    println!("Mirage - a tiny markdown Compiler written by Anirudh Rowjee");
    println!("Version {}", the_version);
}

fn main() {
    usage();
}
