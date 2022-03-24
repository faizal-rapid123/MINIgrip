use std::fs;
use std::env;
use std::process;
struct Config
{
    query:String,
    filename:String

}
impl Config
{
   fn new (args:&[String])->Result<Config, & 'static str>
   {
       if args.len()<3
       {
           Err("enter whole query")
       }
       else
       {
           let query = args[1].clone(); // ownership will be lost so we are using clone method
           let filename = args[2].clone();
          return  Ok(Config{query,filename});
       }
   }

}

fn main()
{
    let args: Vec<String> = env::args().collect(); // collect information from user
    let config = Config::new(&args).unwrap_or_else(|err|
        {
       println!("Problem parsing arguments: {}", err);
       process::exit(1);
   });

   println!("Searching for {}", config.query);
   println!("In file {}", config.filename);

   let contents = fs::read_to_string(config.filename)
       .expect("Something went wrong reading the file");

}

