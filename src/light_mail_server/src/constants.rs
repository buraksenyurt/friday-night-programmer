use std::fmt::{Display, Formatter};

pub enum Command {
    Hello,
    Hello2,
    HelloResponse,
    From,
    To,
    Data,
    Bye,
    Ok,
    Quit,
    Unrecognized,
    EndData,
    ServiceNotAvailable,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Hello => write!(f, "HELO"),
            Command::Hello2 => write!(f, "EHLO"),
            Command::HelloResponse => write!(f, "250 Hello\r\n"),
            Command::From => write!(f, "MAIL FROM"),
            Command::To => write!(f, "RCPT TO"),
            Command::Data => write!(f, "DATA"),
            Command::Bye => write!(f, "221 Bye\r\n"),
            Command::Ok => write!(f, "250 OK\r\n"),
            Command::Quit => write!(f, "QUIT"),
            Command::Unrecognized => write!(f, "500 Unrecognized command\r\n"),
            Command::EndData => write!(f, "354 End data with <CR><LF>.<CR><LF>\r\n"),
            Command::ServiceNotAvailable => write!(f, "421 Service Not Available\r\n"),
        }
    }
}

pub const BUFFER_SIZE: usize = 1024;
pub const EMAIL_DIR: &str = "temp";
