/// Loops forever around a one second [`std::thread::sleep`] call.
/// # Example
/// ```no_run
/// # use spellrs::petrificus_totalus;
/// petrificus_totalus!();
/// let a = 5; // This code will never execute, as the thread is paralyzed!
/// ```
#[macro_export]
macro_rules! petrificus_totalus {
    () => {
        loop {
            ::std::thread::sleep(::std::time::Duration::from_secs(1))
        }
    };
}

/// Alias for [`std::process::exit`]`(1)`.
/// # Example
/// ```no_run
/// # use spellrs::avada_kedavra;
/// avada_kedavra!();
/// let a = 5; // This code will never execute, as the program is dead!
/// ```
#[macro_export]
macro_rules! avada_kedavra {
    () => {
        ::std::process::exit(1)
    };
}

/// Alias for [`std::mem::drop`].
/// # Examples
/// Drop the return value of an expression:
/// ```
/// # use spellrs::obliviate;
/// obliviate!(vec![0; 5]);
/// ```
/// Drop a variable:
/// ```compile_fail
/// # use spellrs::obliviate;
/// let x = vec![0; 5];
/// obliviate!(x);
/// // no longer possible to reference x
/// println!("{x:?}");
/// ```
#[macro_export]
macro_rules! obliviate {
    ($memory:expr) => {
        ::std::mem::drop($memory)
    };
}

/// Constructs the given type using either the `default()`
/// or `new(<optional args>)` functions.
/// Calling it with `erecto!(type)` results in the former, while
/// `erecto!(type: <optional args>)` results in the latter.
/// # Examples
/// ```
/// # use spellrs::erecto;
/// #[derive(Debug, Default, PartialEq)]
/// struct Thing {
///     x: u8,
/// }
///
/// impl Thing {
///     fn new(x: u8) -> Self {
///         Self { x }
///     }
/// }
///
/// assert_eq!(erecto!(u8), 0);
/// assert_eq!(erecto!(String), String::default());
/// assert_eq!(erecto!(String:), String::new());
/// assert_eq!(erecto!(Thing), Thing::default());
/// assert_eq!(erecto!(Thing: 5), Thing::new(5));
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

/// Alias for dereferencing.
/// # Example
/// ```
/// # use spellrs::accio;
/// let x = 5;
/// let y = &x;
/// assert_eq!(accio!(y), x);
///
/// let a = vec![0; 5];
/// assert_eq!(accio!(a.get(0).unwrap()), 0);
/// ```
#[macro_export]
macro_rules! accio {
    ($x:expr) => {
        *$x
    };
}

/// Appends `.clone()`.
/// # Example
/// ```
/// # use spellrs::geminio;
/// let a = vec![0; 5];
/// let b = geminio!(a);
/// drop(a);
/// assert_eq!(b, vec![0; 5]);
/// ```
#[macro_export]
macro_rules! geminio {
    ($object:ident) => {
        $object.clone()
    };
}

/// Alias for [`std::pin::Pin::new`].
/// # Example
/// ```
/// # use spellrs::immobulus;
/// let mut val = 5;
/// let pinned = immobulus!(&mut val);
/// let r = std::pin::Pin::into_inner(pinned);
/// assert_eq!(*r, 5);
/// ```
#[macro_export]
macro_rules! immobulus {
    ($item:expr) => {
        ::std::pin::Pin::new($item)
    };
}

/// Appends `.expect(message)` if given a message, otherwise appends `.unwrap()`.
/// # Examples
/// ```
/// # use spellrs::expecto_patronum;
/// expecto_patronum!(u8::try_from(5));
/// ```
/// ```should_panic
/// # use spellrs::expecto_patronum;
/// expecto_patronum!(u8::try_from(-5), "Here be Dementors!");
/// ```
#[macro_export]
macro_rules! expecto_patronum {
    ($danger:expr, $message:expr) => {
        $danger.expect($message)
    };
    ($danger:expr) => {
        $danger.unwrap()
    };
}

/// Appends `.lock().unwrap()`.
/// # Example
/// ```
/// # use spellrs::colloportus;
/// use std::sync::Mutex;
/// let door = Mutex::new(5);
/// let guard = colloportus!(door);
/// ```
#[macro_export]
macro_rules! colloportus {
    ($door:ident) => {
        $door.lock().unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_obliviate() {
        let x = vec![0; 5];
        obliviate!(x);
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
                Self {
                    b,
                    y4: x,
                    ..Default::default()
                }
            }
        }
        assert_eq!(erecto!(Thing), Thing::default());
        assert_eq!(erecto!(Thing: true, 5), Thing::new(true, 5));
        let b = true;
        let x = 5;
        assert_eq!(erecto!(Thing: b, x), Thing::new(b, x));
        assert_eq!(
            erecto!(Thing: 5 != 2, 2_u8.pow(5)),
            Thing::new(5 != 2, 2_u8.pow(5))
        );
        assert_eq!(erecto!(Thing: 5 != 2, x), Thing::new(5 != 2, x));
        assert_eq!(erecto!(String:), String::new());
    }

    #[test]
    fn practice_geminio() {
        let a = vec![0; 5];
        let b = geminio!(a);
        assert_eq!(a, b);
        drop(a);
        assert_eq!(b, vec![0; 5]);
    }

    #[test]
    fn practice_immobulus() {
        let mut val = 5;
        let pinned = immobulus!(&mut val);
        let r = std::pin::Pin::into_inner(pinned);
        assert_eq!(*r, 5);
    }

    #[test]
    fn practice_expecto_patronum() {
        expecto_patronum!(u8::try_from(5));
    }

    #[test]
    fn practice_colloportus() {
        let door = std::sync::Mutex::new(5);
        let _guard = colloportus!(door);
    }
}
