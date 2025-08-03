use microrust_inject::{get_instance, inject_singleton};

struct MyStruct {}

#[inject_singleton(new)]
impl MyStruct {
    pub fn new() -> Self {
        println!("MyStruct:new() called");
        Self {}
    }

    async fn print(&self) {
        println!("Hello, MyStruct");
    }
}

#[async_std::main]
async fn main() {
    let inst = get_instance::<MyStruct>();
    //    let inst: Arc<MyStruct> = get_instance();
    inst.print().await;
}
