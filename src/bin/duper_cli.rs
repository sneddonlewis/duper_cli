use clap::Parser;

fn main() {
    println!("Duper");
    let opts = duper::opts::Opts::parse();
    println!("{:?}", opts);
}