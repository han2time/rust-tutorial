use std::env;
use std::fs;
use std::process;
use std::error::Error;

/* 12.2
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("검색어: {}", query);
    println!("대상 파일: {}", filename);


    let contents = fs::read_to_string(filename)
        .expect("파일을 찾지 못했습니다.");

    println!("파일 내용: \n{}", contents);
}
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });

    println!("검색어: {}", config.query);
    println!("대상 파일: {}", config.filename);

    // run(config)

    if let Err(e) = run(config) {
        println!("애플리케이션 에러: {}", e);
        process::exit(1);
    }


    // let contents = fs::read_to_string(config.filename)
    //     .expect("파일을 찾지 못했습니다.");

    // println!("파일 내용: \n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

/*
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {query, filename}
}
*/

/*
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("필요한 인수가 지정되지 않았습니다.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config {query, filename}
    }
}
*/

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }
}

/*
fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("파일을 찾지 못했습니다.");

    println!("파일 내용: \n{}", contents);
}
*/

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("파일 내용: \n{}", contents);

    Ok(())
}

