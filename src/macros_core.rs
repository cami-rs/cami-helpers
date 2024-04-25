// @TODO Unify this with cami_wrap_tuple, if possible: they differ in default derives only.
//
// @TODO here & at cami_partial_ord etc: support Higher Trait Bounds <- nomicon
//
// @TODO Try to generate #[doc(...)] and inject $T into this doc comment.
//
// @TODO the same for cami_wrap_struct etc.
/// A zero cost (transparent) wrapper that implements [::core::cmp::PartialEq] forwarding to
/// [camigo::CamiPartialEq] and [::core::cmp::Ord] forwarding to [camigo::CamiOrd] of `T`.
///
/// These implementations are useful, and for many data types it may speed up searches etc.
/// (anything based on comparison).
///
/// [::core::cmp::PartialEq] is implemented NOT by forwarding to [::core::cmp::PartialEq]'s
/// `eq` and `ne` of `T`, but by forwarding to[camigo::CamiOrd]'s `cmp_local`] and
/// `cmp_non_local`` of `T` instead. (Hence `T` itself doesn't need to be
/// [::core::cmp::PartialEq] or [::core::cmp::Ord].)

#[macro_export]
macro_rules! core_wrap_tuple {
    // An INTERNAL rule
    (@
     [$($($derived:path),+)?]
     $struct_vis:vis
     $struct_name:ident
     $(<$($generic:tt $(: $bound:tt)?),+>)?
     (
     $field_vis:vis
     $T:ty
     )
     $(where $($left:ty : $right:tt),+)?
    ) => {
        /// A zero cost (transparent) wrapper struct around a given type. For use with other macros
        /// from this crate.
        $(#[derive($($derived),+)])?
        #[repr(transparent)]
        $struct_vis struct $struct_name $(<$($generic $(: $bound)?),+>)?
        (
            $field_vis $T
        )
        $(where $($left : $right),+)?
        ;
    };

    // The following prevents recursion on incorrect macro invocation
    (@
     $($tt:tt)+
    ) => {
        INCORRECT_MACRO_INVOCATION
        $($tt:tt)+
    };

    // NOT adding Clone/Debug to $derived
    ([$($($derived:path),+)?]
     $($tt:tt)+
    ) => {
        core_wrap_tuple! {
            @
            [$($($derived),+)?]
            $($tt)+
        }
    };

    // Default the derived trait impls
    ($($tt:tt)+) => {
        core_wrap_tuple! {
            @
            [
            ::core::clone::Clone, ::core::fmt::Debug
            ]
            $($tt)+
        }
    };
}

/// Implement [core::cmp::PartialEq] for type `T` that implements[crate::CamiPartialEq].
///
/// There is no corresponding macro for [core::cmp::Eq]. Implement it if you see fit.
#[macro_export]
macro_rules! core_partial_eq {
    ($wrapper_name:ident <$generics:tt> $T:ty) => {
        impl<$generics> ::core::cmp::PartialEq for $wrapper_name<$T>
        where
            $T: camigo::CamiPartialEq,
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                (!T::LOCALITY.has_local() || self.t.eq_local(&other.t))
                    && (!T::LOCALITY.has_non_local() || self.t.eq_non_local(&other.t))
            }

            #[inline]
            fn ne(&self, other: &Self) -> bool {
                T::LOCALITY.has_local() && !self.t.eq_local(&other.t)
                    || T::LOCALITY.has_non_local() && !self.t.eq_non_local(&other.t)
            }
        }
    };
}

/// Implement [core::cmp::PartialOrd] for type `T` that implements[crate::CamiPartialOrd].
#[macro_export]
macro_rules! core_partial_ord {
    ($wrapper_name:ident <$generics:tt> $T:ty) => {
        impl<$generics> ::core::cmp::PartialOrd for $wrapper_name<$T>
        where
            $T: camigo::CamiOrd,
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                Some(self.t.cmp_full(&other.t))
            }

            #[inline]
            fn lt(&self, other: &Self) -> bool {
                self.t.cmp_full(&other.t) == ::core::cmp::Ordering::Less
            }
            #[inline]
            fn le(&self, other: &Self) -> bool {
                self.t.cmp_full(&other.t) != ::core::cmp::Ordering::Greater
            }
            #[inline]
            fn gt(&self, other: &Self) -> bool {
                self.t.cmp_full(&other.t) == ::core::cmp::Ordering::Greater
            }
            #[inline]
            fn ge(&self, other: &Self) -> bool {
                self.t.cmp_full(&other.t) != ::core::cmp::Ordering::Less
            }
        }
    };
}

/// Implement [core::cmp::Ord] for type `T` that implements[crate::CamiOrd].
#[macro_export]
macro_rules! core_ord {
    ($wrapper_name:ident <$generics:tt> $T:ty) => {
        impl<$generics> ::core::cmp::Ord for $wrapper_name<$T>
        where
            $T: camigo::CamiOrd,
        {
            #[inline]
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.t.cmp_full(&other.t)
            }
        }
    };
}
