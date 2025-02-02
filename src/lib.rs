use chrono::Timelike;

fn get_time() -> String {
    let now = chrono::Local::now();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();
    format!("{:02}:{:02}:{:02}", hour, minute, second)
}

/// `Simple` just provides the warning level without the date
pub mod simple {
    use calorie::color::*;
    use calorie::modifiers::RESET;
    /// Logs a message with a custom level and color.
    ///
    /// # Arguments
    /// * `level` - A string representing the log level (e.g., "CUSTOM").
    /// * `color` - An ANSI color code to style the log message.
    /// * `msg` - The message to be logged.
    ///
    /// # Example
    /// ```ignore
    /// Simple::extra("ALERT", "\x1b[31m", "This is a custom alert");
    /// ```
    pub fn extra(level: &str, color: &str, msg: &str) {
        println!("{color}[{level}]{} {msg}", RESET);
    }

    pub fn warn(msg: &str) {
        println!("{}[WARNING]{} {msg}", YELLOW, RESET);
    }

    pub fn error(msg: &str) {
        println!("{}[ERROR]{} {msg}", RED, RESET);
    }

    pub fn fatal(msg: &str) {
        println!("{}[FATAL]{} {msg}", MAGENTA, RESET);
    }

    pub fn info(msg: &str) {
        println!("{}[INFO]{} {msg}", CYAN, RESET);
    }

    pub fn success(msg: &str) {
        println!("{}[SUCCESS]{} {msg}", GREEN, RESET);
    }

    pub fn debug(msg: &str) {
        println!("{}\x1b[2m[DEBUG]{} {msg}", WHITE, RESET);
    }
}

pub mod logger {
    use crate::get_time;
    use calorie::color::*;
    use calorie::modifiers::RESET;
    /// Logs a message with a custom level and color.
    ///
    /// # Arguments
    /// * `level` - A string representing the log level (e.g., "CUSTOM").
    /// * `color` - An ANSI color code to style the log message.
    /// * `msg` - The message to be logged.
    ///
    /// # Example
    /// ```ignore
    /// logger::extra("ALERT", "\x1b[31m", "This is a custom alert");
    /// ```
    pub fn extra(level: &str, color: &str, msg: &str) {
        let time = get_time();
        println!("{color}[{time}] [{level}]{} {msg}", RESET);
    }

    pub fn warn(msg: &str) {
        let time = get_time();
        println!("{}[{time}] [WARNING]{} {msg}", YELLOW, RESET);
    }

    pub fn error(msg: &str) {
        let time = get_time();
        println!("{}[{time}] [ERROR]{} {msg}", RED, RESET);
    }

    pub fn fatal(msg: &str) {
        let time = get_time();
        println!("{}[{time}] [FATAL]{} {msg}", MAGENTA, RESET);
    }

    pub fn info(msg: &str) {
        let time = get_time();
        println!("{}[{time}] [INFO]{} {msg}", CYAN, RESET);
    }

    pub fn success(msg: &str) {
        let time = get_time();
        println!("{}[{time}] [SUCCESS]{} {msg}", GREEN, RESET);
    }

    pub fn debug(msg: &str) {
        let time = get_time();
        println!("{}\x1b[2m[{time}] [DEBUG]{} {msg}", WHITE, RESET);
    }
}

#[cfg(test)]
mod tests {
    use calorie::color::BRIGHT_GREEN;

    use super::*;

    #[test]
    fn it_works() {
        logger::extra("TEST", BRIGHT_GREEN, "THIS IS A TEST");
        simple::extra("TEST", BRIGHT_GREEN, "THIS IS A TEST");
        logger::warn("test");
        logger::fatal("test");
        simple::error("test");
    }
}
