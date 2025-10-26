/*
    DataStore trait'i basit bir veri deposu soyutlaması sağlar.
    Associated type ile veri tipinin belirtilmesi zorunlu kılınır.

    InMemoryStore yapısı, string türünden verileri bellekte saklayan bir veri deposu.
    DataStore trait'ini implemente ediyor ve ilişkili tip olarak String kullanacağını somut bir şekilde belirtiyor.
*/
#[allow(dead_code)]
trait DataStore {
    type Item; // Associated type tanımı

    fn save(&mut self, item: Self::Item);
    fn read(&self, id: u32) -> Option<Self::Item>;
}
struct InMemoryStore {
    items: Vec<String>,
}
impl DataStore for InMemoryStore {
    type Item = String; // Somut tür belirtimi. Artık Item türü String dolayısıyla InMemoryStore sadece String türünden verilerle çalışır.

    fn save(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn read(&self, id: u32) -> Option<Self::Item> {
        self.items.get(id as usize).cloned()
    }
}

/*
    Generic tür parametreleri ile aynı soyutlamayı yapıyoruz.
    Burada ilişkili tip yerine generic tür parametresi T kullanılıyor.
    Dikkat edileceği üzere type şeklinde bir tanımlama yok. Dolayısıyla bu trait'i implemente etmek isteyen bir yapı,
    hangi türü kullanacağını her seferinde belirtmek zorunda. Elbette bazen bu esnekliğe ihtiyaç duyuyoruz.
*/
#[allow(dead_code)]
trait GenericDataStore<T> {
    fn save(&mut self, item: T);
    fn read(&self, id: u32) -> Option<T>;
}
struct GenericInMemoryStore<T> {
    items: Vec<T>,
}

impl<T: Clone> GenericDataStore<T> for GenericInMemoryStore<T> {
    fn save(&mut self, item: T) {
        self.items.push(item);
    }

    fn read(&self, id: u32) -> Option<T> {
        self.items.get(id as usize).cloned()
    }
}

fn main() {
    let mut store = InMemoryStore { items: vec![] };
    store.save("connection string".to_string());
    store.save("minio address".to_string());
    for item in store.items.iter() {
        println!("Loaded from InMemoryStore: {}", item);
    }

    let mut generic_store = GenericInMemoryStore { items: vec![] };
    generic_store.save(42);
    generic_store.save(100);
    if let Some(item) = generic_store.read(1) {
        println!("Loaded from GenericInMemoryStore: {}", item);
    }
}
