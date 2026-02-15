use async_trait::async_trait;
use microrust_inject::{
    get_instance_mutex, get_instance_rwlock, get_trait_instance_mutex, get_trait_instance_rwlock,
    inject_trait_mutex, inject_trait_rwlock, singleton_mutex, singleton_rwlock,
};

struct MyStructMutex {
    counter: i32,
}

struct MyStructRwLock {
    counter: i32,
}

#[async_trait]
trait MyTraitMutex: Send + Sync {
    async fn print_counter(&self);
    async fn increment_counter(&mut self);
}

#[async_trait]
trait MyTraitRwLock: Send + Sync {
    async fn print_counter(&self);
    async fn increment_counter(&mut self);
}

#[singleton_mutex(new)]
impl MyStructMutex {
    pub fn new() -> Self {
        println!("MyStructOuterMutex::new() called");
        Self { counter: 0 }
    }

    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter);
    }

    async fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

#[singleton_rwlock(new)]
impl MyStructRwLock {
    pub fn new() -> Self {
        println!("MyStructOuterRwLock::new() called");
        Self { counter: 0 }
    }

    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter);
    }

    async fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

#[inject_trait_mutex]
impl MyTraitMutex for MyStructMutex {
    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter);
    }

    async fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

#[inject_trait_rwlock]
impl MyTraitRwLock for MyStructRwLock {
    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter);
    }

    async fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

#[async_std::main]
async fn main() {
    let inst = get_instance_mutex::<MyStructMutex>();
    //    let inst: Arc<MyStructOuterMutex> = get_instance();
    inst.lock().unwrap().print_counter().await;
    inst.lock().unwrap().increment_counter().await;
    inst.lock().unwrap().print_counter().await;

    let inst = get_trait_instance_mutex::<dyn MyTraitMutex>();
    //    let inst: Arc<dyn MyTraitOuterMutex> = get_trait_instance();
    inst.lock().unwrap().print_counter().await;
    inst.lock().unwrap().increment_counter().await;
    inst.lock().unwrap().print_counter().await;

    let inst = get_instance_rwlock::<MyStructRwLock>();
    //    let inst: Arc<MyStructOuterRwLock> = get_instance();
    inst.read().unwrap().print_counter().await;
    inst.write().unwrap().increment_counter().await;
    inst.read().unwrap().print_counter().await;

    let inst = get_trait_instance_rwlock::<dyn MyTraitRwLock>();
    //    let inst: Arc<dyn MyTraitOuterRwLock> = get_trait_instance();
    inst.read().unwrap().print_counter().await;
    inst.write().unwrap().increment_counter().await;
    inst.read().unwrap().print_counter().await;
}
