// @TODO consider removing completely
#[macro_export]
macro_rules! core_wrap_struct {
    // NOT adding Clone/Debug
    //
    // @TODO remove "<T>"
    ($struct_name:ident) => {
        core_wrap_struct! { $struct_name <T> T}
    };
    ($struct_name:ident <$generics:tt> $T:ty) => {
        core_wrap_struct! { [::core::clone::Clone, ::core::fmt::Debug] $struct_name <$generics> t $T}
    };
    // NOT adding Clone/Debug
    //
    // @TODO "where" (bounds) part
    //
    // @TODO here & at cami_partial_ord etc: support Higher Trait Bounds <- nomicon
    ([$($($derived:path),+)?] $struct_name:ident <$generics:tt> $t:ident $T:ty) => {
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
        $(#[derive($($derived),+)])?
        #[repr(transparent)]
        pub struct $struct_name<$generics> {
            $t: $T,
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
