use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Bad arguments");
        process::exit(1);
    }
    let fname = &args[1];

    let js = fs::read_to_string(fname)
        .expect("Something went wrong reading the file");

    let html = format!("<!DOCTYPE html><html><head><title>{}</title><script src=\"https://cdnjs.cloudflare.com/ajax/libs/svg.js/3.0.12/svg.min.js\" integrity=\"sha256-IMCBafn5pmCrLYioAYBxLvBirr8BSVB3ocKfzKhHOz0=\" crossorigin=\"anonymous\"></script></head><body><div id=\"d\"></div><script type=\"text/javascript\">SVG.on(document,'DOMContentLoaded',function(){{var draw = SVG(document.getElementById('d')).size(\"100%\", \"100%\");{}}});</script></body></html>", fname, js);
    println!("{}", html);
}
