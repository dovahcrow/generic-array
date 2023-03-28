//! Implementation for `arr!` macro.

use super::ArrayLength;
use core::ops::Add;
use typenum::U1;

/// Helper trait for `arr!` macro
pub trait AddLength<T, N: ArrayLength>: ArrayLength {
    /// Resulting length
    type Output: ArrayLength;
}

impl<T, N1, N2> AddLength<T, N2> for N1
where
    N1: ArrayLength + Add<N2>,
    N2: ArrayLength,
    <N1 as Add<N2>>::Output: ArrayLength,
{
    type Output = <N1 as Add<N2>>::Output;
}

/// Helper type for `arr!` macro
pub type Inc<T, U> = <U as AddLength<T, U1>>::Output;

#[doc(hidden)]
#[macro_export]
macro_rules! arr_impl {
    (@replace_expr $e:expr) => { 1 };
    (@count_ty) => { $crate::typenum::U0 };
    (@count_ty $val:expr$(, $vals:expr)* $(,)?) => { $crate::typenum::Add1<$crate::arr_impl!(@count_ty $($vals),*)> };
    ($($x:expr),*) => ({
        const __INPUT_LENGTH: usize = 0 $(+ $crate::arr_impl!(@replace_expr $x) )*;
        type __OutputLength = $crate::arr_impl!(@count_ty $($x),*);

        #[inline(always)]
        const fn __do_transmute<T, N: $crate::ArrayLength>(arr: [T; __INPUT_LENGTH]) -> $crate::GenericArray<T, N> {
            unsafe { $crate::transmute(arr) }
        }

        const _: [(); <__OutputLength as $crate::typenum::Unsigned>::USIZE] = [(); __INPUT_LENGTH];

        __do_transmute::<_, __OutputLength>([$($x),*])
    });
    ($x:expr; $N: ty) => ({
        const __INPUT_LENGTH: usize = <$N as $crate::typenum::Unsigned>::USIZE;

        #[inline(always)]
        const fn __do_transmute<T, N: $crate::ArrayLength>(arr: [T; __INPUT_LENGTH]) -> $crate::GenericArray<T, N> {
            unsafe { $crate::transmute(arr) }
        }

        __do_transmute::<_, $N>([$x; __INPUT_LENGTH])
    });
}

/// Macro allowing for easy generation of Generic Arrays.
/// Example: `let test = arr![1, 2, 3];`
///
/// Type-inference works similarly to `vec![]`
///
/// Can be used in `const` contexts.
#[macro_export]
macro_rules! arr {
    ($($x:expr),* $(,)*) => ( $crate::arr_impl!($($x),*) );
    ($x:expr; $N:ty)     => ( $crate::arr_impl!($x; $N) );
}

mod doctests_only {
    ///
    /// # With ellision
    ///
    /// Testing that lifetimes aren't transmuted when they're ellided.
    ///
    /// ```compile_fail
    /// #[macro_use] extern crate generic_array;
    /// fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
    ///     arr![a as &A][0]
    /// }
    /// ```
    ///
    /// ```rust
    /// #[macro_use] extern crate generic_array;
    /// fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'a A {
    ///     arr![a][0]
    /// }
    /// ```
    ///
    /// # Without ellision
    ///
    /// Testing that lifetimes aren't transmuted when they're specified explicitly.
    ///
    /// ```compile_fail
    /// #[macro_use] extern crate generic_array;
    /// fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
    ///     arr![a][0]
    /// }
    /// ```
    ///
    /// ```compile_fail
    /// #[macro_use] extern crate generic_array;
    /// fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
    ///     arr![a][0]
    /// }
    /// ```
    ///
    /// ```rust
    /// #[macro_use] extern crate generic_array;
    /// fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'a A {
    ///     arr![a][0]
    /// }
    /// ```
    #[allow(dead_code)]
    pub enum DocTests {}
}
