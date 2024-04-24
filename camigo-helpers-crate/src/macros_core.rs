// @TODO consider removing completely
#[macro_export]
macro_rules! core_wrap_struct {
    // NOT adding Clone/Debug
    ($struct_name:ident) => {
        core_wrap_struct! { $struct_name <T> T}
    };
    ($struct_name:ident <$generics:tt> $T:ty) => {
        core_wrap_struct! { [::core::clone::Clone, ::core::fmt::Debug] $struct_name <$generics> t $T}
    };
    // NOT adding Clone/Debug
    ([$($($derived:path),+)?] $struct_name:ident <$generics:tt> $t:ident $T:ty) => {
        /// @TODO replace $item_type in this doc:
        ///
        /// A (zero cost/low cost) wrapper & bridge that implements [::core::cmp::PartialEq]
        /// forwarding to [camigo::CamiPartialEq] and [::core::cmp::Ord] forwarding to [camigo::CamiOrd]
        /// of `$item_type`.
        ///
        /// These implementations are useful, and for many data types it may speed up searches etc.
        /// (anything based on comparison).
        ///
        /// NO cache-specific benefit for [camigo::Slice]'s cache-aware methods (`binary_search_cf`
        /// etc.) themselves!
        ///
        /// Usable for BENCHMARKING. It allows us to run slice's `binary_search`:
        /// `<[$item_type]>::binary_search(self, given)` using the full comparison, and benchmark
        /// against cache-aware [camigo::Slice]'s `binary_search_cf` etc.
        ///
        /// [::core::cmp::PartialEq] is implemented NOT by forwarding to [::core::cmp::PartialEq]'s
        /// `eq` and `ne` of `$item_type`, but by forwarding to[camigo::CamiOrd]'s `cmp_local`] and
        /// `cmp_non_local`` of `$item_type` instead. (Hence `$item_type` itself doesn't need to be
        /// [::core::cmp::PartialEq] or [::core::cmp::Ord].)
        $(#[derive($($derived),+)])?
        #[repr(transparent)]
        pub struct $struct_name<$generics> {
            $t: $T,
        }
    };
}

/// Implement [core::cmp::PartialEq] for type `T` that implements[crate::CamiPartialEq].
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

// Not really necessary, but let's have it for consistency.
/// Implement [core::cmp::Eq] for type `T` that implements[crate::CamiPartialEq].
#[macro_export]
macro_rules! core_eq {
    ($wrapper_name:ident <$generics:tt> $T:ty) => {
        impl<$generics> ::core::cmp::Eq for $wrapper_name<$T> where $T: camigo::CamiPartialEq {}
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
