mod mymodule;

mod myinlinemodule {
    pub(crate) fn hello_world() -> String {
        "Hello, world!".into()
    }
}

fn main() {
    println!("Hello, world!");
    mymodule::hello_world();
    mymodule::mysubmodule::hello_world2();
}
