use std::collections::HashMap;

use crate::Error::{CouldNotSee, CouldNotSpeak, SomethingElse};
use crate::SubSystemError::Whatever;

mod mymodule;
mod fib;

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Error {
    CouldNotSpeak,
    CouldNotListen,
    CouldNotSee,
    SomethingElse(SubSystemError),
}

// type Error = String;
// const (
// MyErrorOne = "foo";
// MyErrorTwo = "bar";
// )
//

#[derive(Debug, Clone)]
enum SubSystemError {
    Whatever
}

/// denne eier vi ikke
fn sub_command() -> Result<usize, SubSystemError> {
    Err(Whatever)
}

fn communicate() -> Result<Vec<usize>, Error> {
    sub_command()
        .inspect_err(|err| println!("subsystem returned error: {:?}", err))
        .map_err(SomethingElse)
        .map(|number| vec![number])
}

fn ask() -> Result<Option<usize>, Error> {
    let numbers = communicate()?;
    if numbers.len() == 0 {
        Ok(None)
    } else {
        Ok(numbers.first().copied())
    }
}

fn main() -> Result<(), Error> {
    let mut hashmap: HashMap<String, String> = HashMap::new();

    hashmap.insert("key".into(), "value".into());
    hashmap.insert("key2".into(), "value2".into());

    println!("{:?}", hashmap);

    type Lol = HashMap<String, String>;

    let values: Lol = hashmap.iter()
        .filter(|(key, value)| {
            key.starts_with("k") && value.starts_with("v")
        })
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();

    println!("{:?}", values);

    match ask() {
        Ok(Some(_number)) => {}
        Ok(None) => {}
        Err(err) => {
            match err {
                CouldNotSpeak => {}
                CouldNotSee => {}
                SomethingElse(_) => {}
                Error::CouldNotListen => {}
            }
        }
    }

    Ok(())

    // if err:=communicate(); err != nil {
    // return err;
    // }

    // func foo(a *int) {
    // x := *a;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test() {
        let _ = main();
        assert_eq!(1, 2);
    }
}