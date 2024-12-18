//! Do you think Rust macros are a bit like magic? I do too!
//!
//! With this crate you can live your wizard dreams right in your source code.
//! It aliases some common (and some less common) code snippets to macros
//! named after thematically appropriate spells from Harry Potter.
//!
//! This enables you to cast [`geminio!(item)`](geminio) instead of forcing you to call [`item.clone()`](core::clone::Clone::clone).
//! ```
//! # use code_spells::{accio, erecto, obliviate, expecto_patronum, geminio};
//! let v1 = vec![erecto!(i32); 5];
//! let mut v2 = geminio!(&v1);
//! obliviate!(v1);
//! accio!(expecto_patronum!(v2.get_mut(0), "Dementors B-gone!")) = 5;
//! ```
//! Also aliases `unsafe` to the macro [`unforgivable!`](unforgivable),
//! because what could be more unforgivable than undefined behaviour?  

/// Alias for [`std::thread::sleep`](std::thread::sleep).
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

/// Alias for [`panic!`].
/// # Example
/// ```no_run
/// # use code_spells::avada_kedavra;
/// avada_kedavra!("Lily Potter");
/// let lily_potter = "continue"; // This code will never execute, as the program is dead!
/// ```
#[macro_export]
macro_rules! avada_kedavra {
    ($($arg:tt)*) => {
        panic!($($arg)*)
    };
}

/// Alias for [`Drop::drop`](core::mem::drop).
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
        ::core::mem::drop($memory)
    };
}

/// Constructs the given type using either the [`Default::default`](core::default::Default::default)
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
        <$t as ::core::default::Default>::default()
    };
    ($t:ty: $($arg:expr),*) => {
        <$t>::new( $($arg,)* )
    };
}

/// Alias for dereferencing. This does not use the [`Deref`](core::ops::Deref) or [`DerefMut`](core::ops::DerefMut) traits,
/// but prepends `*` to the start of the given expression.
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

/// Alias for [`Clone::clone`](core::clone::Clone::clone).
/// # Example
/// ```
/// # use code_spells::geminio;
/// let a = vec![0; 5];
/// let b = geminio!(&a);
/// drop(a);
/// assert_eq!(b, vec![0; 5]);
/// ```
#[macro_export]
macro_rules! geminio {
    ($object:expr) => {
        ::core::clone::Clone::clone($object)
    };
}

/// Alias for [`Pin::new`](core::pin::Pin::new).
/// # Example
/// ```
/// # use code_spells::immobulus;
/// let mut val = 5;
/// let pinned = immobulus!(&mut val);
/// let r = core::pin::Pin::into_inner(pinned);
/// assert_eq!(*r, 5);
/// ```
#[macro_export]
macro_rules! immobulus {
    ($item:expr) => {
        ::core::pin::Pin::new($item)
    };
}

/// Appends `.expect(message)` if given a message, otherwise appends `.unwrap()`.
/// # Examples
/// ```
/// # use code_spells::expecto_patronum;
/// # use std::convert::TryFrom;
/// expecto_patronum!(u8::try_from(5));
/// ```
/// ```should_panic
/// # use code_spells::expecto_patronum;
/// # use std::convert::TryFrom;
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

/// Alias for [`Mutex::lock`](std::sync::Mutex::lock).
/// # Example
/// ```
/// # use code_spells::colloportus;
/// use std::sync::Mutex;
/// let door = Mutex::new(5);
/// let guard_result = colloportus!(&door);
/// ```
#[macro_export]
macro_rules! colloportus {
    ($door:expr) => {
        ::std::sync::Mutex::lock($door)
    };
}

/// Alias for [`Box::leak`](std::boxed::Box::leak). The item is still there, it's just invisible. Can be revealed with [`aparecium!`](aparecium).
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
/// This crate allows that function to be cast with [`aparecium!`].
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

/// Alias for [`Box::from_raw`](std::boxed::Box::from_raw). Useful if you have made something invisible with [`evanesco!`](evanesco).
/// This is `unsafe` as revealing something invisible might not be what the invisible thing wants,
/// and it might attack you and cause undefined behaviour.
/// # Example
/// ```
/// # use code_spells::{evanesco, aparecium};
/// let a: &mut Vec<i32> = evanesco!(Box::new(vec![5; 100]));
/// assert_eq!(unsafe { aparecium!(a) }, Box::new(vec![5; 100]));
/// ```
#[macro_export]
macro_rules! aparecium {
    ($item:expr) => {
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

/// Alias for [`Result::unwrap_or`](core::result::Result::unwrap_or) and [`Result::unwrap_or_else`](core::result::Result::unwrap_or_else).
/// Automatically chooses [`unwrap_or_else`](core::result::Result::unwrap_or_else) if given a closure,
/// and [`unwrap_or`](core::result::Result::unwrap_or) if given an expression that is not a closure.
/// # Example
/// ```
/// # use code_spells::reparo;
/// fn foo(x: u8) -> Result<u8, u8> {
///     if x < 125 {
///         Ok(x)
///     } else {
///         Err(x)
///     }
/// }
/// let five = 5;
/// assert_eq!(reparo!(foo(5), five), 5); // unwrap_or
/// assert_eq!(reparo!(foo(255), u8::MAX), u8::MAX); // unwrap_or
/// assert_eq!(reparo!(foo(255), |_| 5), 5); // unwrap_or_else
/// let primes = vec![2, 3, 5];
/// assert_eq!(reparo!(foo(255), move |_| primes.into_iter().sum()), 10); // unwrap_or_else
/// ```
/// # Note
/// If the second argument is the name of a function this macro will not work.
/// ```compile_fail
/// # use code_spells::reparo;
/// # fn foo(x: u8) -> Result<u8, u8> {if x < 125 { Ok(x) } else { Err(x) } }
/// fn ten() -> u8 { 10 }
/// assert_eq!(reparo!(foo(255), ten), 10);
/// ```
/// We can make it work by converting the function to a closure
/// ```
/// # use code_spells::reparo;
/// # fn foo(x: u8) -> Result<u8, u8> {if x < 125 { Ok(x) } else { Err(x) } }
/// # fn ten() -> u8 { 10 }
/// assert_eq!(reparo!(foo(255), |_| ten()), 10); // uses unwrap_or_else
/// ```
/// or by calling the function in the macro.
/// ```
/// # use code_spells::reparo;
/// # fn foo(x: u8) -> Result<u8, u8> {if x < 125 { Ok(x) } else { Err(x) } }
/// # fn ten() -> u8 { 10 }
/// assert_eq!(reparo!(foo(255), ten()), 10); // uses unwrap_or
/// ```
#[macro_export]
macro_rules! reparo {
    ($result:expr, move |$arg_name:pat_param| $body:expr) => {
        ::core::result::Result::unwrap_or_else($result, move |$arg_name| $body)
    };
    ($result:expr, |$arg_name:pat_param| $body:expr) => {
        ::core::result::Result::unwrap_or_else($result, |$arg_name| $body)
    };
    ($result:expr, $alt:expr) => {
        ::core::result::Result::unwrap_or($result, $alt)
    };
}

/// Alias for unsafe. What could be more unforgivable than undefined behaviour?
/// # Example
/// ```
/// # use code_spells::unforgivable;
/// use core::num::NonZeroU8;
/// // Forgivability: `new_unchecked` is UB if the argument is zero, but two is not zero.
/// const two: NonZeroU8 = unforgivable! { NonZeroU8::new_unchecked(2) };
/// assert_eq!(two.get(), 2);
/// ```
#[macro_export]
macro_rules! unforgivable {
    ($($code:tt)+) => {
        unsafe {
            $($code)+
        }
    };
}

/// Alias for [`Vec::reserve`](std::vec::Vec::reserve).
/// # Example
/// ```
/// # use code_spells::capacious_extremis;
/// let mut police_box = Vec::<i32>::new();
/// capacious_extremis!(&mut police_box, 5);
/// assert!(police_box.capacity() >= 5);
/// let r = &mut police_box;
/// capacious_extremis!(r, 10);
/// assert!(police_box.capacity() >= 10);
/// ```
#[macro_export]
macro_rules! capacious_extremis {
    (&mut $vec:ident, $capacity:expr) => {
        ::std::vec::Vec::reserve(&mut $vec, $capacity)
    };
    ($vec:ident, $capacity:expr) => {
        ::std::vec::Vec::reserve($vec, $capacity)
    };
}

/// Alias for [`mem::transmute`](core::mem::transmute). Disregard the rules, force the type system to do what you want!
/// # Safety
/// This spell is unforgivable for a reason, see the documentation of [`mem::transmute`](core::mem::transmute) for more details.
/// # Examples
/// ```
/// # use code_spells::{imperio, unforgivable};
/// let a = [0_u8; 4];
/// let b: u32 = unforgivable! { imperio!(a) };
/// assert_eq!(b, 0);
/// let mut c = unforgivable! { imperio!(b, u32 => [u8; 4]) };
/// assert_eq!(c, [0; 4]);
/// let d = &mut c;
/// assert_eq!(unforgivable! { imperio!(d, &mut [u8; 4] => &[u8; 4]) }, &c)
/// ```
/// Force a pointer to become a function pointer!
/// ```
/// # use code_spells::{imperio, unforgivable};
/// fn foo() -> i32 { 0 }
/// let pointer = foo as *const ();
/// let function = unforgivable! {
///     imperio!(pointer, *const () => fn() -> i32)
/// };
/// assert_eq!(function(), 0);
/// ```
#[macro_export]
macro_rules! imperio {
    // Elision
    ($will:expr) => {
        ::core::mem::transmute($will)
    };
    // Pure type to type
    ($will:expr, $src:ty => $dst:ty) => {
        ::core::mem::transmute::<$src, $dst>($will)
    };
    // Attempt to match against any expression
    ($will:expr, $src:expr => $dst:expr) => {
        ::core::mem::transmute::<$src, $dst>($will)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

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
        let b = geminio!(&a);
        assert_eq!(a, b);
        drop(a);
        assert_eq!(b, vec![0; 5]);
    }

    #[test]
    fn practice_immobulus() {
        let mut val = 5;
        let pinned = immobulus!(&mut val);
        let r = core::pin::Pin::into_inner(pinned);
        assert_eq!(*r, 5);
    }

    #[test]
    fn practice_expecto_patronum() {
        expecto_patronum!(u8::try_from(5));
    }

    #[test]
    fn practice_colloportus() {
        let door = std::sync::Mutex::new(5);
        let _guard = colloportus!(&door);
    }

    #[test]
    fn practice_evanesco_and_apericium() {
        let a = Box::new(vec![5; 100]);
        let b: &mut Vec<i32> = evanesco!(a);
        assert_eq!(unsafe { aparecium!(b) }, Box::new(vec![5; 100]));
    }

    #[test]
    fn practice_reparo() {
        fn foo(x: u8) -> Result<u8, u8> {
            if x < 125 {
                Ok(x)
            } else {
                Err(x)
            }
        }
        let five = 5;
        fn identity(x: u8) -> u8 {
            x
        }
        assert_eq!(reparo!(foo(5), five), 5);
        assert_eq!(reparo!(foo(255), u8::MAX), u8::MAX);
        assert_eq!(reparo!(foo(255), |_| 5), 5);
        assert_eq!(reparo!(foo(255), |_| identity(10)), 10);
    }

    #[test]
    fn practice_capacious_extremis() {
        let mut a = Vec::<i32>::new();
        let b = &mut a;
        capacious_extremis!(b, 5);
        capacious_extremis!(&mut a, 10);
        assert!(a.capacity() >= 10);
    }

    #[test]
    fn practice_imperio() {
        let a = [0_u8; 4];
        let b: u32 = unforgivable! { imperio!(a) };
        assert_eq!(b, 0);
        let c = unforgivable! { imperio!(b, u32 => [u8; 4]) };
        assert_eq!(c, [0; 4]);
    }
}
