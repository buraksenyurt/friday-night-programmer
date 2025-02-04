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
    EndDataReply,
    ServiceNotAvailable,
    ServerReady,
    MessageAccepted,
    ErrorSimulated
}

impl Command {
    pub fn as_str(&self) -> &'static str {
        match self {
            Command::Hello => "HELO",
            Command::Hello2 => "EHLO",
            Command::HelloResponse => "250 Hello\r\n",
            Command::From => "MAIL FROM",
            Command::To => "RCPT TO",
            Command::Data => "DATA",
            Command::Bye => "221 Bye\r\n",
            Command::Ok => "250 OK\r\n",
            Command::Quit => "QUIT",
            Command::Unrecognized => "500 Unrecognized command\r\n",
            Command::EndDataReply => "354 End data with <CR><LF>.<CR><LF>\r\n",
            Command::ServiceNotAvailable => "421 Service Not Available\r\n",
            Command::ServerReady => "220 Mock SMTP Server Ready\r\n",
            Command::MessageAccepted => "250 Message accepted for delivery\r\n",
            Command::ErrorSimulated => "421 Error simulated for recipient.",
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub const BUFFER_SIZE: usize = 1024;
pub const EMAIL_DIR: &str = "temp";
pub const INVALID_MAIL: &str = "thereisnomail@nowhere.nowhere";
