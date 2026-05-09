use std::marker::PhantomData;

pub fn run() {
    let some_box = SomeBox::new(String::from("Phantom of the Opera"));
    println!("Created SomeBox with value: '{}'", unsafe { &*some_box.p });
}

struct SomeBox<T> {
    p: *mut T, // bellekteki veriyi işaret eden saf işaretçi(sahiplik yok)
    /*
    PhantomData<T> burada SomeBox<T> tipinin T tipine sahip olduğunu belirtmek için kullanılır.
    Bu, Rust'ın sahiplik ve yaşam süresi kurallarını doğru bir şekilde uygulamasına yardımcı olur.
    Yani drop check mekanizması, SomeBox<T> türünün T türüne sahip olduğunu bilir ve
    bu türün yaşam süresi boyunca SomeBox<T> türünün de geçerli olduğunu varsayar.
     */
    _marker: PhantomData<T>,
}

impl<T> SomeBox<T> {
    fn new(value: T) -> Self {
        let ptr = Box::into_raw(Box::new(value)); // Box'u saf işaretçiye dönüştürüyoruz
        SomeBox {
            p: ptr,
            _marker: PhantomData, // Çalışma zamanında 0 byte yer kaplar ki bunu biliyoruz artık
        }
    }
}

impl<T> Drop for SomeBox<T> {
    fn drop(&mut self) {
        // raw pointer kullandığımız için veriyi geri okuma işlemi güvenli değildir
        // Dolayısıyla bir unsafe bloğu içinde raw pointer'ı geri alarak belleği serbest bırakmamız gerekir
        unsafe {
            let _ = Box::from_raw(self.p);
        }
        println!("SomeBox dropped and memory freed.");
    }
}
