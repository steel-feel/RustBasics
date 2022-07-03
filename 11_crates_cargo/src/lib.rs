pub mod lib_nat {
  
    use clap::Parser;
    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
  
    struct Args {
        #[clap(short, long, value_parser)]
        name: String,
        /// Number of times to greet
        #[clap(short, long, value_parser, default_value_t = 1)]
        count: u8,
    }

    pub fn intro() {
        println!("I am invoked from a lib");
    }

    pub fn greet_me() {
        let args = Args::parse();

        for _ in 0..args.count {
            println!("Hello {}!", args.name)
        }
    }
    
}
