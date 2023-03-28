/// Loops forever on a sleep (1 sec) call.
#[macro_export]
macro_rules! petrificus_totalus {
    () => {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1))
        }
    }
}

/// Exits the process with error code 1.
#[macro_export]
macro_rules! avada_kedavra {
    () => {
        std::process::exit(1)
    }
}

/// Drops the given variable.
#[macro_export]
macro_rules! obliviate {
    ($memory:ident) => {
        drop($memory)
    }
}

/// Drops the given variable.
#[macro_export]
macro_rules! expelliarmus {
    ($item:ident) => {
        drop($item)
    }
}

/// Drops the given variable.
#[macro_export]
macro_rules! evanesco {
    ($item:ident) => {
        drop($item)
    }
}

/// Constructs the given type using either the `default()`
/// or `new(<optional args>)` functions.
/// Calling it with `erecto(type)` results in the former, while
/// `erecto!(type: <optional args>)` results in the latter.
/// # Examples
/// ```
/// # use spellrs::erecto;
/// assert_eq!(erecto!(u8), 0);
/// assert_eq!(erecto!(String:), String::new());
/// ```
#[macro_export]
macro_rules! erecto {
    ($t:ty) => {
        <$t>::default()
    };
    ($t:ty: $($arg:expr),*) => {
        <$t>::new( $($arg,)* )
    };
}

/// Dereferences the given variable.
/// # Example
/// ```
/// # use spellrs::accio;
/// let x = 5;
/// let y = &x;
/// assert_eq!(accio!(y), x);
/// ```
#[macro_export]
macro_rules! accio {
    ($x:ident) => {
        *$x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_obliviate_and_friends() {
        let x = 5;
        obliviate!(x);
        let y = 5;
        evanesco!(y);
        let z = 5;
        expelliarmus!(z);
    }

    #[test]
    fn practice_accio() {
        let x = 5;
        let y = &x;
        assert_eq!(accio!(y), 5);
    }

    #[test]
    fn practice_erecto() {
        #[derive(Debug, Default, PartialEq)]
        struct Thing {
            x1: f64,
            x2: f32,
            y1: u64,
            y2: u32,
            y3: u16,
            y4: u8,
            y5: i64,
            y6: i32,
            y7: i16,
            y8: i8,
            y9: u128,
            y10: i128,
            y11: usize,
            y12: isize,
            b: bool,
            cs: std::ffi::CString,
            os: std::ffi::OsString,
            s: String,
        }
        impl Thing {
            fn new(b: bool, x: u8) -> Self {
                Self {b, y4: x, ..Default::default()}
            }
        }
        assert_eq!(erecto!(Thing), Thing::default());
        assert_eq!(erecto!(Thing: true, 5), Thing::new(true, 5));
        let b = true;
        let x = 5;
        assert_eq!(erecto!(Thing: b, x), Thing::new(b, x));
        assert_eq!(erecto!(Thing: 5 != 2, 2_u8.pow(5)), Thing::new(5 != 2, 2_u8.pow(5)));
        assert_eq!(erecto!(Thing: 5 != 2, x), Thing::new(5 != 2, x));
    }
}
