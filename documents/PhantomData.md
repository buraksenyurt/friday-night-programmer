# Rust Dilin Phantom Type Kullanımı: PhantomData

Bazı durumlarda bir tipe ekstra bilgiler dahil ederken bu bilgilerin çalışma zamanında *(runtime)* gerçekten de saklanmasını istemeyiz. Kulağa garip bir cümle olarak geldiği aşikar, farkındayım. Bir örnek üzerinden ilerlersek daha anlaşılır olacaktır ama öncesinde temel bilgileri ele alalım. Rust dilinde `PhantomData<T>` şeklinde generic bir yapı bulunuyor. **PhantomData** yapısı ile tanımlanan bir veri çalışma zamanında saklamaz ama derleyici bu türün kullanıldığını bilir ve buna bağlı olarak ownership, borrowing, lifetimes gibi kuralları işletebilir. Zaten bu türe phantom yani "hayalet" denmesinin sebebi de budur; çalışma zamanında var olmayan ama derleyici tarafından bilinen tür olarak ifade edilebilir.

PhantomData türünün en büyük avantajı boyutunun sıfır olmasıdır. Dolayısıyla çalışma zamanında **T** türü için herhangibir bellek tahsisi yapılmaz ve performans açısından herhangi bir ek yük oluşmaz. Bu elbette akıllara `o zaman ne gibi senaryolarda hangi amaçlarla kullanırız?` sorusunu getirir. Temel olarak derleme zamanında bazı doğrulamaların garanti edilmesinin sağlanması istenen senaryoları örnek gösterebiliriz.

Bazı durumlarda **trait**'ler karıştırıldığı da olur. trait'lerde çeşitli türden nesnelerle çalışacak şekilde soyutlamalar *(Abstractions)* yapabiliriz ve fonksiyonlar yazabiliriz. Ancak trait'ler çalışma zamanında da vardır ve **dynamic dispatch** gibi mekanizmaların getirdiği performans maliyetleri bulunur. Eğer trait'ler ile çalışırken türlerin karışması gibi bir durumun önüne geçmek istiyorsak ve bu tür bilgisi çalışma zamanında kullanılmayacaksa **PhantomData** kullanabiliriz. Kısacası, derleme zamanında type-safe bir yaklaşım sağlarken runtime'a taşımamıza gerek olmayan tür bilgileri için **PhantomData** kullanışlıdır.

Ne zaman trait ne zaman PhantomData sorusu ile ilgili olarak güzel bir cümleyi olduğu gibi paylaşmak isterim,

> Trait'ler **What you can do with a type** sorusuna cevap verirken, PhantomData **What kind of thing it is** sorusuna cevap verir.

Ayrıca PhantoData türün ile ilgili şunları aklımızda tutmakta fayda var.

- Çalışma zamanı verilerini doğrulamazlar.
- Sadece derleme zamanında tür seviyesinde kuralların uygulanmasını sağlarlar.
- Boyutları sıfırdır, dolayısıyla çalışma zamanında herhangi bir bellek tahsisi yapılmaz.
- Örneğe göre söz gelimi "Button" ifadesinin gerçekten bir Button türü olduğunu kontrol etmezler. Sadece derleyiciye bu türün kullanıldığını bildirirler.

PhantomData türünün kullanımını daha iyi anlamak için aşağıdaki örnek kodları ele alabiliriz.

```rust
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
```

Bu örnekte kullanılan `Component<Render>` veri yapısı farklı ortamlara render edilebilecek bileşenleri temsil ediyor. Örneğin HTML olarak rende edilecek bir Button veya Linux masaüstü için bir Label gibi. `PhantomData<Render>` kullanarak bir bileşenin hangi ortam için olduğunu derleme zamanında belirtiyoruz. Ancak bu tür bilgisi çalışma zamanında saklanmıyoruz.
