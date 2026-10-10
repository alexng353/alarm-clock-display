use chrono::{DateTime, Utc};

pub enum Action {
    Bootsel,
    Pong,
    SetTime(DateTime<Utc>),
}

pub fn parse_command(line_buf: &[u8], line_len: usize) -> Result<Action, &'static str> {
    let Ok(line) = core::str::from_utf8(&line_buf[..line_len].trim_ascii_end()) else {
        return Err("err not utf8");
    };

    let (cmd, args) = line
        .trim_end()
        .split_once(' ')
        .unwrap_or((line.trim_end(), ""));

    match cmd {
        "bootsel" => {
            return Ok(Action::Bootsel);
        }
        "ping" => return Ok(Action::Pong),
        "time" => {
            let new_time = match args.parse::<i64>() {
                Ok(n) => n,
                Err(_) => return Err("err invalid timestamp\n"),
            };

            let wall_time = DateTime::from_timestamp_secs(new_time).unwrap();
            return Ok(Action::SetTime(wall_time));
        }
        _ => {
            return Err("err cmd invalid\n");
        }
    }
}
