use log::LevelFilter;
use simplelog::{Config, SimpleLogger};

pub fn init() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).expect("Unable to init log");

    log::info!(r"             ('-. .-.               .-')    .-') _    ");
    log::info!(r"            ( OO )  /              ( OO ). (  OO) )   ");
    log::info!(r"  ,----.    ,--. ,--. .-'),-----. (_)---\_)/     '._  ");
    log::info!(r" '  .-./-') |  | |  |( OO'  .-.  '/    _ | |'--...__) ");
    log::info!(r" |  |_( O- )|   .|  |/   |  | |  |\  :` `. '--.  .--' ");
    log::info!(r" |  | .--, \|       |\_) |  |\|  | '..`''.)   |  |    ");
    log::info!(r"(|  | '. (_/|  .-.  |  \ |  | |  |.-._)   \   |  |    ");
    log::info!(r" |  '--'  | |  | |  |   `'  '-'  '\       /   |  |    ");
    log::info!(r"  `------'  `--' `--'     `-----'  `-----'    `--'    ");
}

pub fn pretty_hash(input: &str) -> String {
    hex::encode(urlencoding::decode_binary(input.as_bytes()))
}
