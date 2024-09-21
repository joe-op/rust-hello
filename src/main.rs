use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.name);
}
