#[cfg(debug_assertions)]
pub fn log<T>(message: T)
where
    T: std::fmt::Display,
{
    println!("{message}");
}

#[cfg(not(debug_assertions))]
pub fn log<T>(message: T)
where
    T: std::fmt::Display,
{
    let formatter = syslog::Formatter3164 {
        facility: syslog::Facility::LOG_LOCAL1,
        ..Default::default()
    };

    match syslog::unix(formatter) {
        Err(e) => panic!("impossible to connect to syslog: {:?}", e),
        Ok(mut writer) => {
            writer.err(message).expect("could not write error message");
        }
    }
}
