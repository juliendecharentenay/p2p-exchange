use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
enum LogLevel {
  Off,
  Error,
  Warn,
  Info,
  Debug,
  Trace,
}

impl LogLevel {
  pub fn into(v: &LogLevel) -> log::LevelFilter {
    match v {
      LogLevel::Off   => log::LevelFilter::Off,
      LogLevel::Error => log::LevelFilter::Error,
      LogLevel::Warn  => log::LevelFilter::Warn,
      LogLevel::Info  => log::LevelFilter::Info,
      LogLevel::Debug => log::LevelFilter::Debug,
      LogLevel::Trace => log::LevelFilter::Trace,
    }
  }
}

#[derive(Parser)]
struct Config {
  #[arg(long = "dbPath", default_value = "./test.db3")]
  pub db_filename: String,
  #[arg(long = "log", value_enum)]
  pub log_level: LogLevel,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::parse();
  env_logger::builder().filter_level(LogLevel::into(&config.log_level)).init();
  let app_state = actix_web::web::Data::new(
       signalling::AppStateBuilder::default()
       .filename(config.db_filename)
       .build()?
  );

  actix_web::HttpServer::new(move || {
    actix_web::App::new()
      .app_data(app_state.clone())
      .service(actix_web::web::scope("/api/count/offer").configure(signalling::offer::actix::config_count))
      .service(actix_web::web::scope("/api/count/answer").configure(signalling::answer::actix::config_count))
      .service(actix_web::web::scope("/api/offer").configure(signalling::offer::actix::config))
      .service(actix_web::web::scope("/api/answer").configure(signalling::answer::actix::config))
      .service(actix_web::web::scope("/api/message").configure(signalling::message::actix::config))
  })
  .bind(("127.0.0.1", 8010))?
  .run()
  .await?;
  Ok(())
}
