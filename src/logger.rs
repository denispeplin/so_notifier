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
    match syslog::unix(syslog::Formatter3164::default()) {
        Err(e) => panic!("impossible to connect to syslog: {:?}", e),
        Ok(mut writer) => {
            writer.err(message).expect("could not write error message");
        }
    }
}
