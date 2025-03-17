use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

// Pool'a eklenebilecek nesne davranışlarını tanımlayan bir trait
// Bunu generic constrait eklemek için kullanacağız
trait Poolable {
    fn reset(&mut self);
}

// Havuza bırakılmış nesneleri yöneten veri modeli
struct PooledObject<T: Poolable> {
    inner: Option<T>,
    pool: Arc<Mutex<ObjectPool<T>>>,
}

impl<T: Poolable> Deref for PooledObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<T: Poolable> DerefMut for PooledObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

impl<T: Poolable> Drop for PooledObject<T> {
    fn drop(&mut self) {
        if let Some(mut object) = self.inner.take() {
            object.reset();
            let mut pool = self.pool.lock().unwrap();
            pool.send_back(object); // drop'larda nesneyi havuza iade eden fonksiyonu çağırıyoruz
        }
    }
}

// Nesne havuzu
struct ObjectPool<T: Poolable> {
    objects: Vec<T>,
    max_capacity: usize,
}

impl<T: Poolable> ObjectPool<T> {
    pub fn new(max_capacity: usize) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(ObjectPool {
            objects: Vec::new(),
            max_capacity,
        }))
    }

    pub fn add(&mut self, value: T) {
        if self.objects.len() < self.max_capacity {
            self.objects.push(value);
        } else {
            println!("Pool is full");
        }
    }

    pub fn get(&mut self) -> Option<T> {
        if !self.objects.is_empty() {
            Some(self.objects.remove(0))
        } else {
            None
        }
    }

    pub fn send_back(&mut self, value: T) {
        if self.objects.len() < self.max_capacity {
            self.objects.push(value);
        } else {
            println!("Pool is full, discarding object");
        }
    }
}

// Havuzda kullanacağımız nesne
struct AssetServer {
    id: i16,
    assets: Vec<String>,
    is_active: bool,
}

impl AssetServer {
    fn new(id: i16, assets: Vec<String>) -> Self {
        AssetServer {
            id,
            assets,
            is_active: true,
        }
    }

    fn get_id(&self) -> i16 {
        self.id
    }
}

impl Poolable for AssetServer {
    fn reset(&mut self) {
        self.is_active = true;
        println!("AssetServer({}) is resetting", self.id);
    }
}

fn get_from_pool<T: Poolable>(pool: Arc<Mutex<ObjectPool<T>>>) -> Option<PooledObject<T>> {
    let mut guard = pool.lock().unwrap();
    guard.get().map(|o| PooledObject {
        inner: Some(o),
        pool: Arc::clone(&pool),
    })
}

pub fn run() {
    println!("\nRefactored Object Pooling Sample\n");

    let pool = ObjectPool::<AssetServer>::new(10);

    {
        let mut pool_guard = pool.lock().unwrap();
        for i in 0..5 {
            pool_guard.add(AssetServer::new(
                i,
                vec!["block.png".to_string(), "bricks.png".to_string()],
            ));
        }
    }

    {
        let server_1 = get_from_pool(Arc::clone(&pool)).unwrap();
        let server_2 = get_from_pool(Arc::clone(&pool)).unwrap();

        println!("Server 1 id {}", server_1.get_id());
        println!("Server 2 id {}", server_2.get_id());
    } // scope dışına gelindi. Server nesneleri otomatik olarak düşecek.

    {
        let pool_guard = pool.lock().unwrap();
        println!("Pool now has {} objects", pool_guard.objects.len());
    }

    // Nesnelerin tekrardan havuza döndüğünü ispat etmek için kullanabiliriz
    {
        let server_3 = get_from_pool(Arc::clone(&pool)).unwrap();
        let server_4 = get_from_pool(Arc::clone(&pool)).unwrap();

        println!(
            "Server 3 (id {}) has {} objects.",
            server_3.get_id(),
            server_3.assets.len()
        );
        println!(
            "Server 4(id {}) has {} objects.",
            server_4.get_id(),
            server_4.assets.len()
        );
    }
}
