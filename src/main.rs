mod mymodule;

fn main() {
    println!("Hello, world!");
    mymodule::hello_world();
    mymodule::mysubmodule::hello_world2();
}
