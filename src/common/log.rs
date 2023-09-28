use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use log::LevelFilter;

pub struct AppLog {
}

impl AppLog {
    pub fn trivial_conf(log_level: u8) {
        let level = match log_level {
            1 => LevelFilter::Debug,
            2 => LevelFilter::Info,
            3 => LevelFilter::Warn,
            4 => LevelFilter::Error,
            _ => LevelFilter::Info,
        };

        let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}\n")))
            .build(format!("./temp.log"))
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .appender(Appender::builder()
                          .filter(Box::new(ThresholdFilter::new(level)))
                          .build("stderr", Box::new(stderr)),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stderr")
                    .build(level),
            ).unwrap();

        log4rs::init_config(config).unwrap();
    }

    pub fn log_only_stderr() {
        let level = log::LevelFilter::Debug;

        let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

        let config = Config::builder()
            .appender(Appender::builder()
                          .filter(Box::new(ThresholdFilter::new(level)))
                          .build("stderr", Box::new(stderr)),
            )
            .build(
                Root::builder()
                    .appender("stderr")
                    .build(LevelFilter::Debug),
            ).unwrap();

        log4rs::init_config(config).unwrap();
    }
}
