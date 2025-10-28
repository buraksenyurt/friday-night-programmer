fn main() {
    let mut settings = settings::AppSettings::new(settings::LogLevel::Info);
    println!("Initial Settings: {:?}", settings);

    settings.set_connections(200);
    settings.set_port(9090);
    println!("Settings after update: {:?}", settings);
}

/*
    settings modülünde yer alan LogLevel pub erişim belirleyicisi ile tanımlanmış bir enum'dur.
    Dolayısıyla settings modülü dışından da erişilebilir.
    
    modüle veri yapılarından olan AppSettings struct'ı da dışarıdan erişilebilir çünkü o da
    pub erişim belirleyicisi ile tanımlanmıştır. Ancak, AppSettings struct'ının bazı alanları
    (max_connections ve port) pub olarak tanımlanmadıklarından dışarıdan doğrudan erişilemezler.
    Bu alanlara erişim ve değiştirme işlemleri için public metotlar (getters ve setters) ile sağlanır.

    AppSettings veri yapısı new metodu ile oluşturulurken LogLevel değerini dışarıdan alabilir ancak,
    max_connections ve port alanları varsayılan değerlerle (DEFAULT_MAX_CONNECTIONS ve DEFAULT_PORT)
    başlatılır. Bu sayede, dışarıdan erişilemeyen alanların kontrolü modül içinde tutulmuş olur.
    Yani, bir encapsulation (kapsülleme) sağlanmış olur.
*/
#[allow(dead_code)]
mod settings {

    #[derive(Debug)]
    pub enum LogLevel {
        Error,
        Warn,
        Info,
        Debug,
        Trace,
    }

    #[derive(Debug)]
    pub struct AppSettings {
        pub log_level: LogLevel,
        max_connections: u32,
        port: u16,
    }

    impl AppSettings {
        const DEFAULT_MAX_CONNECTIONS: u32 = 100;
        const DEFAULT_PORT: u16 = 8080;

        pub fn new(log_level: LogLevel) -> Self {
            AppSettings {
                log_level,
                max_connections: Self::DEFAULT_MAX_CONNECTIONS,
                port: Self::DEFAULT_PORT,
            }
        }

        pub fn get_connections(&self) -> u32 {
            self.max_connections
        }

        pub fn set_connections(&mut self, connections: u32) {
            self.max_connections = connections;
        }

        pub fn get_port(&self) -> u16 {
            self.port
        }

        pub fn set_port(&mut self, port: u16) {
            self.port = port;
        }
    }
}
