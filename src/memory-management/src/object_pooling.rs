use std::sync::Arc;
use std::sync::Mutex;

trait Identifiable {
    fn get_id(&self) -> i16;
}

struct AssetServer {
    id: i16,
}

impl AssetServer {
    fn new(value: i16) -> Self {
        AssetServer { id: value }
    }
}

impl Identifiable for AssetServer {
    fn get_id(&self) -> i16 {
        self.id
    }
}

// Havuzdaki nesneleri tutan veri yapısı
// Generic T türü ile çalışır
struct ObjectPool<T> {
    objects: Arc<Mutex<Vec<T>>>, // T anında sadece bir thread'in erişimini garanti etmek için Atomic Reference Counted ve Mutex kullanılmıştır.
    capacity: usize,
}

impl<T> ObjectPool<T> {
    pub fn new(capacity: usize) -> Self {
        ObjectPool {
            objects: Arc::new(Mutex::new(Vec::new())),
            capacity,
        }
    }

    // Ekleme, çekme ve serbest bırakma operasyonlarının tamamında Mutex lock kullanılır
    pub fn add(&mut self, value: T) {
        if self.objects.lock().unwrap().len() < self.capacity {
            self.objects.lock().unwrap().push(value);
        }
    }

    pub fn get(&mut self) -> Option<T> {
        let mut objects = self.objects.lock().unwrap(); // Havuzdaki nesneler kilit konularak çekilir
        if objects.len() > 0 {
            // Eğer havuzda nesne varsa
            return objects.pop(); // sonraki nesne verilir ve ayrıca bu nesne vektör serisinden çıkartılır
        }
        None
    }

    // Bir nesne ile işimiz bittiğinde onu havuza tekrardan yerleştirmek için kullanılan fonksiyon
    pub fn release(&mut self, value: T) {
        if self.objects.lock().unwrap().len() < self.capacity {
            self.objects.lock().unwrap().push(value);
        } else {
            println!("Pool is full");
        }
    }
}

pub fn run() {
    let mut asset_pool: ObjectPool<Box<dyn Identifiable>> =
        ObjectPool::<Box<dyn Identifiable>>::new(10);

    for i in 0..5 {
        asset_pool.add(Box::new(AssetServer::new(i)));
    }

    let server_1 = asset_pool.get().unwrap();
    println!("Server 1 id {}", server_1.get_id());
    let server_2 = asset_pool.get().unwrap();
    println!("Server 2 id {}", server_2.get_id());
    asset_pool.release(server_2);

    for object in asset_pool.objects.lock().unwrap().iter() {
        println!("Asset server id: {}", object.get_id());
    }
}
