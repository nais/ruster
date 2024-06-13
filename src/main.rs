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

    // Immutable vector
    let my_vector = Vec::from([1, 1, 2, 3, 5, 8]);
    // Mutable vector, based off of an immutable one
    let mut new_vector = &mut return_new_fibonacci_13(&my_vector);
    new_vector = add_fibonacci_13(new_vector);

    // Happen to have the same contents!
    assert!(my_vector == *new_vector);
}

/// Return new vector
fn return_new_fibonacci_13(my_vector: &[i32]) -> Vec<i32> {
    [my_vector, &[13]].concat().to_vec()
}

/// Return the same vector
fn add_fibonacci_13(my_vector: &mut Vec<i32>) -> &mut Vec<i32> {
    my_vector.push(13);
    my_vector
}

// Her har du en trait
trait NavDokument {
    fn dokumenter() {
        todo!()
    }
}

// her har du en annen trait, som arver NavDokument traitet
trait ArkiverbartDokument: NavDokument {
    fn arkiver() {
        Self::dokumenter();
        todo!()
    }
}
