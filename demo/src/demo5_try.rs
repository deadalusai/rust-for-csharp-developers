
//The try! macro is defined (Something) like this...

macro_rules! try {
    ($expr:expr) => {
        match $expr {
            Ok(result) => result,
            Err(err) => return Err(err)
        }
    }
}