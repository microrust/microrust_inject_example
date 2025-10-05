use async_trait::async_trait;
use microrust_inject::{get_trait_instance, inject_singleton, inject_trait};

struct MyStruct {}

#[async_trait]
trait MyTrait: Send + Sync {
    async fn print(&self);
}

#[inject_singleton(new)]
impl MyStruct {
    pub fn new() -> Self {
        println!("MyStruct::new() called");
        Self {}
    }
}

#[inject_trait]
impl MyTrait for MyStruct {
    async fn print(&self) {
        println!("Hello, MyTrait for MyStruct");
    }
}

#[async_std::main]
async fn main() {
    let inst = get_trait_instance::<dyn MyTrait>();
    //    let inst: Arc<dyn MyTrait> = get_trait_instance();
    inst.print().await;
}
