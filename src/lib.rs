//! Do you think Rust macros are a bit like magic? I do too!
//! With this crate you can live your wizard dreams right in your source code.
//! It aliases some common (and some less common) code snippets to macros
//! named after thematically appropriate spells from Harry Potter.
//! ```
//! # use code_spells::{accio, erecto, obliviate, expecto_patronum, geminio};
//! let v1 = vec![erecto!(i32); 5];
//! let mut v2 = geminio!(v1);
//! obliviate!(v1);
//! accio!(expecto_patronum!(v2.get_mut(0), "Dementors B-gone!")) = 5;
//! ```

/// Alias for [`std::thread::sleep`].
/// # Example
/// ```
/// # use code_spells::petrificus_totalus;
/// petrificus_totalus!(std::time::Duration::from_secs(1));
/// ```
#[macro_export]
macro_rules! petrificus_totalus {
    ($duration:expr) => {
        ::std::thread::sleep($duration)
    };
}

/// Alias for [`panic!`](panic).
/// # Example
/// ```no_run
/// # use code_spells::avada_kedavra;
/// avada_kedavra!("No!");
/// let lily_potter = "continue"; // This code will never execute, as the program is dead!
/// ```
#[macro_export]
macro_rules! avada_kedavra {
    ($($arg:tt)*) => {
        panic!($($arg)*)
    };
}

/// Alias for [`std::mem::drop`].
/// # Examples
/// Drop the return value of an expression:
/// ```
/// # use code_spells::obliviate;
/// obliviate!(vec![0; 5]);
/// ```
/// Drop a variable:
/// ```compile_fail
/// # use code_spells::obliviate;
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
/// # use code_spells::erecto;
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
/// # use code_spells::accio;
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

/// Alias for [`std::clone::Clone::clone`]`(&input)`.
/// # Example
/// ```
/// # use code_spells::geminio;
/// let a = vec![0; 5];
/// let b = geminio!(a);
/// drop(a);
/// assert_eq!(b, vec![0; 5]);
/// ```
#[macro_export]
macro_rules! geminio {
    ($object:expr) => {
        ::std::clone::Clone::clone(&$object)
    };
}

/// Alias for [`std::pin::Pin::new`].
/// # Example
/// ```
/// # use code_spells::immobulus;
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
/// # use code_spells::expecto_patronum;
/// expecto_patronum!(u8::try_from(5));
/// ```
/// ```should_panic
/// # use code_spells::expecto_patronum;
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

/// Alias for [`std::sync::Mutex::lock`]`(&input).unwrap()`.
/// # Example
/// ```
/// # use code_spells::colloportus;
/// use std::sync::Mutex;
/// let door = Mutex::new(5);
/// let guard = colloportus!(door);
/// ```
#[macro_export]
macro_rules! colloportus {
    ($door:ident) => {
        ::std::sync::Mutex::lock(&$door).unwrap()
    };
}

/// Alias for [`Box::leak`](std::boxed::Box::leak). The item is still there, it's just harder to see.
/// # Examples
/// If the returned pointer is dropped this causes a memory leak. You forgot where you put it, and it's invisible.
/// ```compile_fail
/// # use code_spells::evanesco;
/// let a = Box::new(vec![5; 100]);
/// evanesco!(a);
/// println!("{a:?}");
/// ```
/// ```no_run
/// # use code_spells::evanesco;
/// let ostrich = Box::new(vec![5; 100]);
/// // What do you have there?
/// evanesco!(ostrich);
/// // A smoothie..?
/// ```
/// Using [`Box::from_raw`](std::boxed::Box::from_raw) is one way of getting the item back.
/// This crate allows that function to be cast with [aparecium!](aparecium).
/// ```
/// # use code_spells::{evanesco, aparecium};
/// let a: &mut Vec<i32> = evanesco!(Box::new(vec![5; 100]));
/// assert_eq!(unsafe { aparecium!(a) }, Box::new(vec![5; 100]));
/// ```
#[macro_export]
macro_rules! evanesco {
    ($item:expr) => {
        ::std::boxed::Box::leak($item)
    };
}

/// Alias for [`Box::from_raw`](std::boxed::Box::from_raw). Useful if you have made something invisible with [evanesco!](evanesco).
/// This is unsafe as revealing something invisible might not be what the invisible thing wants,
/// and it might attack you and cause undefined behaviour.
/// # Example
/// ```
/// # use code_spells::{evanesco, aparecium};
/// let a: &mut Vec<i32> = evanesco!(Box::new(vec![5; 100]));
/// assert_eq!(unsafe { aparecium!(a) }, Box::new(vec![5; 100]));
/// ```
#[macro_export]
macro_rules! aparecium {
    ($item:ident) => {
        ::std::boxed::Box::from_raw($item)
    };
}

/// Alias for [`println!`].
/// # Example
/// ```
/// # use code_spells::sonorous;
/// sonorous!("Hello, World!");
/// sonorous!("{} chocolate", "dark");
/// let a = 1 + 1;
/// sonorous!("{a} is not {}", 5);
/// ```
#[macro_export]
macro_rules! sonorous {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {
        println!($($arg)*)
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

    #[test]
    fn practice_evanesco_and_apericium() {
        let a = Box::new(vec![5; 100]);
        let b: &mut Vec<i32> = evanesco!(a);
        assert_eq!(unsafe { aparecium!(b) }, Box::new(vec![5; 100]));
    }
}
