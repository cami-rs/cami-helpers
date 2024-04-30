/// Implement [core::cmp::PartialEq] for type `T` that implements[crate::CamiPartialEq].
///
/// There is no corresponding macro for [core::cmp::Eq]. Implement it if you see fit.
#[macro_export]
macro_rules! core_partial_eq {
    ($wrapper_name:ident <$generics:tt> $T:ty
    ) => {
        impl<$generics> ::core::cmp::PartialEq for $wrapper_name<$T>
        where
            $T: camigo::CamiPartialEq,
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.t.eq_full(&other.t)
            }

            #[inline]
            fn ne(&self, other: &Self) -> bool {
                !self.t.eq_full(&other.t)
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
