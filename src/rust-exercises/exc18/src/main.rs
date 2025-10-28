/// Doğrulanmış ve doğrulanmamış kullanıcıları temsil eden bir enum tanımı
enum AuthenticatedUser {
    /// Doğrulanmış kullanıcı bilgilerini tutar
    Verified { username: String, email: String },
    /// Doğrulanmamış kullanıcı bilgisini temsil eder
    Unverified,
}

/// Kullanıcı bilgilerini temsil eden bir yapı
struct User {
    /// Kullanıcı adı
    username: String,
    /// Kullanıcı e-posta adresi
    email: String,
}

/// Kullanıcıyı doğrulayan bir fonksiyon
/// Eğer kullanıcı adı veya e-posta boş ise None döner.
/// E-posta "@" karakterini içeriyorsa Verified, içermiyorsa Unverified döner.
///
/// # Arguments
/// * `user` - Doğrulanacak kullanıcı bilgilerini içeren referans
/// # Returns
/// * `Option<AuthenticatedUser>` - Doğrulama sonucunu içeren enum
fn authenticate(user: &User) -> Option<AuthenticatedUser> {
    /*
    Çok basit birkaç doğrulama işlemi gerçekleştiriyoruz.
    Bir gerçek hayat senaryosunda elbetteki daha karmaşık doğrulama işlemleri yapılması gerekir.
    Örneğin, e-posta adresinin geçerliliğini kontrol etmek için regex kullanılabilir veya
    kullanıcı adı belirli kurallara göre doğrulanabilir.

    Bu da birden fazla enum varyantının ele alınması anlamına gelir.
    Eğer kodda tek varyantla ilgileniyorsak, match ifadesi kullanmak yerine if let kullanımı daha temiz ve okunabilir olur.
    */
    if user.username.is_empty() || user.email.is_empty() {
        return None;
    }

    if user.email.contains("@") {
        Some(AuthenticatedUser::Verified {
            username: user.username.clone(),
            email: user.email.clone(),
        })
    } else {
        Some(AuthenticatedUser::Unverified)
    }
}

fn main() {
    let user = User {
        username: "john_doe".to_string(),
        email: "john_doe@example.com".to_string(),
    };

    let auth_user = authenticate(&user);

    // Bad Practice: match ifadesi kullanımında tüm durumları ele almak zorundayız
    match auth_user {
        Some(AuthenticatedUser::Verified { username, email }) => {
            println!("Username: {}, Email: {}", username, email);
        }
        Some(AuthenticatedUser::Unverified) => {
            println!("User is unverified.");
        }
        _ => {
            println!("Authentication failed.");
        }
    }

    let user = User {
        username: "jessica".to_string(),
        email: "jessica@example.com".to_string(),
    };
    let auth_user = authenticate(&user);

    // Good Practice: if let kullanımı
    /*
    Sadece Verified durumunu ele almak istediğimiz bir senaryoda match ifadesi kullandığımız için tüm
    durumları kontrol etmek zorunda kalıyoruz. Bu da kodun gereksiz yere karmaşıklaşmasına neden oluyor.
    if let kullanımı ile sadece ilgilendiğimiz durumu ele alabiliriz ve kod daha temiz ve okunabilir olur.
    */
    if let Some(AuthenticatedUser::Verified { username, email }) = auth_user {
        println!("Username: {}, Email: {}", username, email);
    } else {
        println!("User is unverified.");
    }

    /*
    Aşağıdaki kullanımda sadece None durumunu ele alıyoruz.
    Diğer durumlarla ilgilenmiyoruz. Bu durumda match ifadesi yerine if let kullanımı daha temiz ve okunabilir olur.

    Lakin buna cargo clippy redundant pattern matching, consider using `is_none()` uyarısı verir.
    is_none() kullanımı daha da temiz ve okunabilirdir.
    */
    let user = User {
        username: "".to_string(),
        email: "".to_string(),
    };
    let auth_user = authenticate(&user);
    // if let None = auth_user {
    //     println!("Authentication failed.");
    // }

    if auth_user.is_none() {
        println!("Authentication failed.");
    }
}
