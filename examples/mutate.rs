use async_trait::async_trait;
use microrust_inject::{
    get_instance, get_instance_mutex, get_instance_rwlock, get_trait_instance,
    get_trait_instance_mutex, get_trait_instance_rwlock, inject_singleton, inject_singleton_mutex,
    inject_singleton_rwlock, inject_trait, inject_trait_mutex, inject_trait_rwlock,
};
use std::sync::{Mutex, RwLock};

struct MyStructInnerMutex {
    counter: Mutex<i32>,
}

struct MyStructInnerRwLock {
    counter: RwLock<i32>,
}

struct MyStructOuterMutex {
    counter: i32,
}

struct MyStructOuterRwLock {
    counter: i32,
}

#[async_trait]
trait MyTraitInnerMutex: Send + Sync {
    async fn print_counter(&self);
    async fn increment_counter(&self);
}

#[async_trait]
trait MyTraitInnerRwLock: Send + Sync {
    async fn print_counter(&self);
    async fn increment_counter(&self);
}

#[async_trait]
trait MyTraitOuterMutex: Send + Sync {
    async fn print_counter(&self);
    async fn increment_counter(&mut self);
}

#[async_trait]
trait MyTraitOuterRwLock: Send + Sync {
    async fn print_counter(&self);
    async fn increment_counter(&mut self);
}

#[inject_singleton(new)]
impl MyStructInnerMutex {
    pub fn new() -> Self {
        println!("MyStructInnerMutex:new() called");
        Self {
            counter: Mutex::new(0),
        }
    }

    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter.lock().unwrap());
    }

    async fn increment_counter(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }
}

#[inject_singleton(new)]
impl MyStructInnerRwLock {
    pub fn new() -> Self {
        println!("MyStructInnerRwLock:new() called");
        Self {
            counter: RwLock::new(0),
        }
    }

    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter.read().unwrap());
    }

    async fn increment_counter(&self) {
        let mut counter = self.counter.write().unwrap();
        *counter += 1;
    }
}

#[inject_singleton_mutex(new)]
impl MyStructOuterMutex {
    pub fn new() -> Self {
        println!("MyStructOuterMutex:new() called");
        Self { counter: 0 }
    }

    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter);
    }

    async fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

#[inject_singleton_rwlock(new)]
impl MyStructOuterRwLock {
    pub fn new() -> Self {
        println!("MyStructOuterRwLock:new() called");
        Self { counter: 0 }
    }

    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter);
    }

    async fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

#[inject_trait]
impl MyTraitInnerMutex for MyStructInnerMutex {
    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter.lock().unwrap());
    }

    async fn increment_counter(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }
}

#[inject_trait]
impl MyTraitInnerRwLock for MyStructInnerRwLock {
    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter.read().unwrap());
    }

    async fn increment_counter(&self) {
        let mut counter = self.counter.write().unwrap();
        *counter += 1;
    }
}

#[inject_trait_mutex]
impl MyTraitOuterMutex for MyStructOuterMutex {
    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter);
    }

    async fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

#[inject_trait_rwlock]
impl MyTraitOuterRwLock for MyStructOuterRwLock {
    async fn print_counter(&self) {
        println!("Counter: {:?}", self.counter);
    }

    async fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

#[async_std::main]
async fn main() {
    let inst = get_instance::<MyStructInnerMutex>();
    //    let inst: Arc<MyStructInnerMutex> = get_instance();
    inst.print_counter().await;
    inst.increment_counter().await;
    inst.print_counter().await;

    let inst = get_trait_instance::<dyn MyTraitInnerMutex>();
    //    let inst: Arc<dyn MyTraitInnerMutex> = get_trait_instance();
    inst.print_counter().await;
    inst.increment_counter().await;
    inst.print_counter().await;

    let inst = get_instance::<MyStructInnerRwLock>();
    //    let inst: Arc<MyStructInnerRwLock> = get_instance();
    inst.print_counter().await;
    inst.increment_counter().await;
    inst.print_counter().await;

    let inst = get_trait_instance::<dyn MyTraitInnerRwLock>();
    //    let inst: Arc<dyn MyTraitInnerRwLock> = get_trait_instance();
    inst.print_counter().await;
    inst.increment_counter().await;
    inst.print_counter().await;

    let inst = get_instance_mutex::<MyStructOuterMutex>();
    //    let inst: Arc<MyStructOuterMutex> = get_instance();
    inst.lock().unwrap().print_counter().await;
    inst.lock().unwrap().increment_counter().await;
    inst.lock().unwrap().print_counter().await;

    let inst = get_trait_instance_mutex::<dyn MyTraitOuterMutex>();
    //    let inst: Arc<dyn MyTraitOuterMutex> = get_trait_instance();
    inst.lock().unwrap().print_counter().await;
    inst.lock().unwrap().increment_counter().await;
    inst.lock().unwrap().print_counter().await;

    let inst = get_instance_rwlock::<MyStructOuterRwLock>();
    //    let inst: Arc<MyStructOuterRwLock> = get_instance();
    inst.read().unwrap().print_counter().await;
    inst.write().unwrap().increment_counter().await;
    inst.read().unwrap().print_counter().await;

    let inst = get_trait_instance_rwlock::<dyn MyTraitOuterRwLock>();
    //    let inst: Arc<dyn MyTraitOuterRwLock> = get_trait_instance();
    inst.read().unwrap().print_counter().await;
    inst.write().unwrap().increment_counter().await;
    inst.read().unwrap().print_counter().await;
}
