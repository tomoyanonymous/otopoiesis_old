use std::sync::atomic;
extern crate atomic_float;

pub trait ToAtomic {
    type InnerT: Sized;
    type AtomicT: Sized + From<Self::InnerT>;
}
pub trait Loadable<T> {
    fn load(&self, order: atomic::Ordering) -> T;
}
// macro for creating this wrapper type for each atomic type.
// impl ToAtomic for i32 {
//     type T = atomic::AtomicI32;
// }
macro_rules! make_atomic_wrapper {
    ($tfrom:ty, $tto:ty) => {
        impl ToAtomic for $tfrom {
            type InnerT = $tfrom;
            type AtomicT = $tto;
        }
        impl Loadable<$tfrom> for $tto {
            fn load(&self, order: atomic::Ordering) -> $tfrom {
                self.load(order)
            }
        }
    };
}

make_atomic_wrapper!(bool, atomic::AtomicBool);

make_atomic_wrapper!(i8, atomic::AtomicI8);
make_atomic_wrapper!(u8, atomic::AtomicU8);
make_atomic_wrapper!(i16, atomic::AtomicI16);
make_atomic_wrapper!(u16, atomic::AtomicU16);
make_atomic_wrapper!(i32, atomic::AtomicI32);
make_atomic_wrapper!(u32, atomic::AtomicU32);
make_atomic_wrapper!(i64, atomic::AtomicI64);
make_atomic_wrapper!(u64, atomic::AtomicU64);
make_atomic_wrapper!(isize, atomic::AtomicIsize);
make_atomic_wrapper!(usize, atomic::AtomicUsize);

make_atomic_wrapper!(f32, atomic_float::AtomicF32);
make_atomic_wrapper!(f64, atomic_float::AtomicF64);

// dsp parameter wrapper that can be accessed from multiple threads

pub struct Params<T: ToAtomic> {
    value: T::AtomicT,
}

impl<T: ToAtomic> Params<T>
where
    T::AtomicT: Loadable<T::InnerT>,
{
    pub fn new(v: T::InnerT) -> Self {
        Self {
            value: T::AtomicT::from(v),
        }
    }
    fn from(from: T::InnerT) -> Self {
        Self::new(from)
    }
    pub fn into_inner(&self) -> T::InnerT {
        self.value.load(atomic::Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    macro_rules! make_test {
        ($ans:expr,$type:ty,$testname:ident) => {
            #[test]
            fn $testname() {
                let param = Params::<$type>::from($ans);
                assert_eq!(param.into_inner(), $ans);
            }
        };
    }
    macro_rules! make_test_int {
        ($type:ty,$testname:ident) => {
            make_test! {100,$type,$testname}
        };
    }

    make_test! {true,bool,param_atomic_test_bool}
    make_test_int! {i8,param_atomic_test_i8}
    make_test_int! {i16,param_atomic_test_i16}
    make_test_int! {i32,param_atomic_test_i32}
    make_test_int! {i64,param_atomic_test_i64}

    make_test_int! {u8,param_atomic_test_u8}
    make_test_int! {u16,param_atomic_test_u16}
    make_test_int! {u32,param_atomic_test_u32}
    make_test_int! {u64,param_atomic_test_u64}

    make_test_int! {isize,param_atomic_test_isize}
    make_test_int! {usize,param_atomic_test_usize}

    make_test! {100.,f32,param_atomic_test_f32}
    make_test! {100.,f64,param_atomic_test_f64}
}
