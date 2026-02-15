use std::sync::Arc;

use microrust::inject::{get_instance, singleton};

struct MyStruct {}

struct MyStruct2 {
    ms: Arc<MyStruct>,
}

#[singleton(new)]
impl MyStruct {
    pub fn new() -> Self {
        println!("MyStruct::new() called");
        Self {}
    }

    async fn print(&self) {
        println!("Hello, MyStruct");
    }
}

#[singleton(new)]
impl MyStruct2 {
    pub fn new(ms: Arc<MyStruct>) -> Self {
        println!("MyStruct2::new() called");
        Self { ms: ms }
    }

    async fn print(&self) {
        println!("Hello, MyStruct2");
        self.ms.print().await;
    }
}

#[async_std::main]
async fn main() {
    let inst: Arc<MyStruct> = get_instance();
    inst.print().await;

    let inst2: Arc<MyStruct2> = get_instance();
    inst2.print().await;
}
