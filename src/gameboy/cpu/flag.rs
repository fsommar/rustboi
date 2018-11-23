use super::RegisterF;
use std::{
    self,
    convert::From,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

/// `flag::Value` is an alternative view of `RegisterF` for a specific flag.
///
/// For example, `flag::Value<flag::Z>` will only be allowed to change and read the bit associated
/// with flag Z.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub(crate) struct Value<Flag> {
    is: RegisterF,
    phantom: PhantomData<Flag>,
}

/// `Flag` is a marker trait for the seven flags in register F.
///
/// Each flag has its own offset in the 8 bit register F, and this trait gives meaning to flag
/// structs without increasing their memory usage -- they're still zero sized.
pub(crate) trait Flag {
    fn offset() -> u8;
}

/// Simplifies the construction of an empty flag struct with an offset implementation of the
/// `Flag` trait.
macro_rules! flags_with_offsets {
    ($($flag:ident : $value:expr),*) => {$(
        #[derive(Debug, Copy, Clone)]
        pub struct $flag;
        impl Flag for $flag {
            fn offset() -> u8 {
                $value
            }
        }
    )*}
}

flags_with_offsets![Z: 7, N: 6, H: 5, C: 4, _3: 3, _2: 2, _1: 1, _0: 0];

/// This should be the only implementation for `Value`.
///
/// Since `Value` always will be a (mutable) borrow of `RegisterF`, it gains direct access to its
/// boolean flags. As a matter of fact, `Value` _is_ `RegisterF` but with limited access to its
/// internals, only allowing access via the type-specified `Flag`.
impl<T: Flag> Value<T> {
    // Internally used constructor to abstract the necessary transmute to convert from register F.
    fn new_mut(rf: &mut RegisterF) -> &mut Value<T> {
        unsafe { std::mem::transmute(rf) }
    }

    // Internally used constructor to abstract the necessary transmute to convert from register F.
    fn new(rf: &RegisterF) -> &Value<T> {
        unsafe { std::mem::transmute(rf) }
    }

    /// Sets flag `T` in register F to the provided bool.
    ///
    /// `set` and `reset` should be preferred whenever possible, since they more clearly show
    /// intent.
    pub fn set_bool(&mut self, value: bool) {
        let rf: &mut RegisterF = unsafe { std::mem::transmute(self) };
        let offset = <T as Flag>::offset();
        *(&mut rf.f) = rf.f & !(1u8 << offset) | ((value as u8) << offset);
    }

    /// Reads the flag as a bool.
    pub fn as_bool(&self) -> bool {
        let rf: &RegisterF = unsafe { std::mem::transmute(self) };
        let offset = <T as Flag>::offset();
        (rf.f >> offset) & 1u8 != 0
    }

    /// Toggles the flag, i.e. `false` => `true` or vice versa.
    pub fn toggle(&mut self) {
        let rf: &mut RegisterF = unsafe { std::mem::transmute(self) };
        let offset = <T as Flag>::offset();
        *(&mut rf.f) = rf.f ^ (1u8 << offset);
    }

    /// Sets the flag to `true`.
    pub fn set(&mut self) {
        self.set_bool(true)
    }

    /// Resets the flag to `false`.
    pub fn reset(&mut self) {
        self.set_bool(false)
    }
}

impl<T: Flag> Index<T> for RegisterF {
    type Output = Value<T>;

    // T is only used as an indicator of which `Flag` to use; the (zero-sized) value is never used.
    fn index(&self, _: T) -> &Value<T> {
        Value::new(self)
    }
}

// I haven't figured out a way to plug into the mutable index borrow to allow setting a flag in
// register F, i.e. `register.f()[flag::Z] = true`. The current syntax is to use one of the mutable
// methods, e.g. `register.f()[flag::Z].set()` for the same effect as the previous assignment.
//
// I don't see it as a big drawback, but it'd be interesting to see if it's possible out of mere
// curiousity if nothing else.
impl<T: Flag> IndexMut<T> for RegisterF {
    // T is only used as an indicator of which `Flag` to use; the (zero-sized) value is never used.
    fn index_mut(&mut self, _: T) -> &mut Value<T> {
        Value::new_mut(self)
    }
}

impl<T: Flag> From<Value<T>> for bool {
    fn from(value: Value<T>) -> Self {
        value.as_bool()
    }
}

#[test]
fn test_flag_offsets() {
    assert_eq!(7, <Z as Flag>::offset());
    assert_eq!(6, <N as Flag>::offset());
    assert_eq!(5, <H as Flag>::offset());
    assert_eq!(4, <C as Flag>::offset());
    assert_eq!(3, <_3 as Flag>::offset());
    assert_eq!(2, <_2 as Flag>::offset());
    assert_eq!(1, <_1 as Flag>::offset());
    assert_eq!(0, <_0 as Flag>::offset());
}

#[test]
fn test_set_bool() {
    let mut rf: RegisterF = Default::default();
    {
        let fv = Value::<Z>::new_mut(&mut rf);
        fv.set_bool(true);
    }
    assert_eq!(rf, RegisterF { f: 0b1000_0000 });
    {
        let fv = Value::<Z>::new_mut(&mut rf);
        fv.set_bool(false);
    }
    assert_eq!(rf, RegisterF { f: 0 });
}

#[test]
fn test_toggle() {
    let mut rf: RegisterF = Default::default();
    rf[Z].toggle();
    assert_eq!(rf, RegisterF { f: 0b1000_0000 });
    rf[N].toggle();
    assert_eq!(rf, RegisterF { f: 0b1100_0000 });
    rf[Z].toggle();
    assert_eq!(rf, RegisterF { f: 0b0100_0000 });
}

#[test]
fn test_as_bool() {
    let mut rf: RegisterF = Default::default();
    assert_eq!(false, rf[Z].into());
    assert_eq!(false, rf[Z].as_bool());
    {
        let fv = Value::<Z>::new_mut(&mut rf);
        fv.set_bool(true);
    }
    assert_eq!(true, rf[Z].into());
    assert_eq!(true, rf[Z].as_bool());
}

#[test]
fn test_set_reset() {
    let mut rf: RegisterF = Default::default();
    {
        let fv = Value::<Z>::new_mut(&mut rf);
        fv.set();
    }
    assert_eq!(rf, RegisterF { f: 0b1000_0000 });
    {
        let fv = Value::<Z>::new_mut(&mut rf);
        fv.reset();
    }
    assert_eq!(rf, RegisterF { f: 0 });
}

#[test]
fn test_sizes() {
    assert_eq!(0, std::mem::size_of::<Z>());
    assert_eq!(1, std::mem::size_of::<Value<Z>>());
}
