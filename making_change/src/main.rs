use clap::Parser;

//cargo run -- -v <VALUE> to run the CLI from the project

#[derive(Parser,Debug)]
#[command(name = "making_change", about = "A tool to calculate the number of coins needed to make change.")]
struct Args {

    ///Integer value to make change in cents
    #[clap(short, long)]
    value: u32,
}



fn main() {
    let args = Args::parse();
    let mut value = args.value;
    let change_values = vec![500,100,25,10,5,1];
    let mut coins_used = 0;
    for coin_value in change_values {
        if value>= coin_value {
            coins_used += value/coin_value;
            value %= coin_value;
        } 
    }
    println!("The total coins used to make change is {}",coins_used);
}
