use std::marker::PhantomData;

fn main() {
    let post_button = create_button("Submit");
    let name_label = create_label("Name:");
    let input_field = create_text_field("Enter your name");
    let desktop_button = create_button_linux("Click Me");

    println!(
        "Created a '{}' with content: '{}'",
        post_button.get_type(),
        post_button.content
    );
    println!(
        "Created a '{}' with content: '{}'",
        name_label.get_type(),
        name_label.content
    );
    println!(
        "Created a '{}' with content: '{}'",
        input_field.get_type(),
        input_field.content
    );
    println!(
        "Created a '{}' with content: '{}'",
        desktop_button.get_type(),
        desktop_button.content
    );

    /*
    Aşağıdaki kullanım çalışma zamanında aşağıdaki gibi bir derleme hatasının üretilmesine sebep olur.

    error[E0308]: mismatched types
    --> src\main.rs:38:19
    |
    38 |     render_button(&input_field);
    |     ------------- ^^^^^^^^^^^^ expected `&Component<Html>`, found `&Component<MobileIos>`
    |     |
    |     arguments to this function are incorrect
    |
    = note: expected reference `&Component<Html>`
                found reference `&Component<MobileIos>`
    */
    // render_button(&input_field);

    render_button(&post_button); // Geçerli Kullanım
}

fn render_button(button: &Component<Html>) {
    println!(
        "Rendering a button into HTML for content: {}",
        button.content
    );
}

struct Html;
struct LinuxDesktop;
struct MobileIos;

struct Component<Render> {
    content: String,
    marker: PhantomData<Render>,
}

impl Component<Html> {
    fn get_type(&self) -> &str {
        "HTML Component"
    }
}

impl Component<LinuxDesktop> {
    fn get_type(&self) -> &str {
        "Linux Desktop Component"
    }
}

impl Component<MobileIos> {
    fn get_type(&self) -> &str {
        "Mobile iOS Component"
    }
}

/*
    Aşağıdaki fonksiyonlar farklı render tipleri için Component örnekleri oluşturuyor.
    PhantomData'yı bileşenin türünü belirtmek için kullanıyoruz ancak bu tür bilgisi çalışma zamanında kullanılmıyor.
    Sıfır maliyet. vtable ve dynamic dispatch kullanılmıyor. Component türü tamamen derleme zamanı için bir takı(tag) olarak işlev görüyor.
*/
fn create_button(content: &str) -> Component<Html> {
    Component {
        content: content.to_string(),
        marker: PhantomData,
    }
}

fn create_button_linux(content: &str) -> Component<LinuxDesktop> {
    Component {
        content: content.to_string(),
        marker: PhantomData,
    }
}

fn create_label(content: &str) -> Component<LinuxDesktop> {
    Component {
        content: content.to_string(),
        marker: PhantomData,
    }
}

fn create_text_field(content: &str) -> Component<MobileIos> {
    Component {
        content: content.to_string(),
        marker: PhantomData,
    }
}
