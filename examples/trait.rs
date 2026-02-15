use async_trait::async_trait;
use microrust_inject::{get_trait_instance, inject_trait, singleton};

struct MyStruct {}

#[async_trait]
trait MyTrait: Send + Sync {
    async fn print(&self);
}

#[singleton(new)]
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
