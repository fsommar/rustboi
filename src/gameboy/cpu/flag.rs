use std::{self, convert::From, marker::PhantomData, ops::{Index, IndexMut}};
use super::RegisterF;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub(super) struct Value<Flag> {
    is: RegisterF,
    phantom: PhantomData<Flag>,
}

pub(super) trait Flag {
    fn offset() -> u8;
}

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

impl<T: Flag> Value<T> {
    fn new_mut(rf: &mut RegisterF) -> &mut Value<T> {
        unsafe { std::mem::transmute(rf) }
    }

    fn new(rf: &RegisterF) -> &Value<T> {
        unsafe { std::mem::transmute(rf) }
    }

    fn set_bool(&mut self, value: bool) {
        let rf: &mut RegisterF = unsafe { std::mem::transmute(self) };
        let offset = <T as Flag>::offset();
        *(&mut rf.f) = rf.f & !(1u8 << offset) | ((value as u8) << offset);
    }

    pub fn as_bool(&self) -> bool {
        let rf: &RegisterF = unsafe { std::mem::transmute(self) };
        let offset = <T as Flag>::offset();
        (rf.f >> offset) & 1u8 != 0
    }

    pub fn toggle(&mut self) {
        let rf: &mut RegisterF = unsafe { std::mem::transmute(self) };
        let offset = <T as Flag>::offset();
        *(&mut rf.f) = rf.f ^ (1u8 << offset);
    }

    pub fn set(&mut self) {
        self.set_bool(true)
    }

    pub fn reset(&mut self) {
        self.set_bool(false)
    }
}

impl<T: Flag> Index<T> for RegisterF {
    type Output = Value<T>;

    fn index(&self, _: T) -> &Value<T> {
        Value::new(self)
    }
}

impl<T: Flag> IndexMut<T> for RegisterF {
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
fn test_registerf_z() {
    let mut rf: RegisterF = Default::default();
    assert_eq!(7, <Z as Flag>::offset());
    {
        let fv = Value::<Z>::new_mut(&mut rf);
        fv.set_bool(true);
    }
    assert_eq!(rf, RegisterF { f: 0b1000_0000 });
    assert_eq!(true, rf[Z].as_bool());
    {
        let fv = Value::<Z>::new_mut(&mut rf);
        fv.set_bool(false);
    }
    assert_eq!(rf, RegisterF { f: 0 });
    rf[Z].toggle();
    assert_eq!(rf, RegisterF { f: 0b1000_0000 });
    rf[Z].toggle();
    assert_eq!(rf, RegisterF { f: 0 });
    assert_eq!(false, rf[Z].into());
}

#[test]
fn test_registerf_z_2() {
    let mut rf: RegisterF = Default::default();
    assert_eq!(7, <Z as Flag>::offset());
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