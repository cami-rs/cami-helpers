/// NOT a part of public API. Only for use by macro-generated code. Subject to change.
///
/// The main benefit: With this, we don't need to capture the wrapped/forwarded type in
/// [cami_partial_eq] & [cami_ord] when we apply those macros to a (`#[repr(transparent)]`) wrapper
/// struct or tuple. See also how we needed `$t_type:ty` (in commit `06cfc12`):
/// <https://github.com/cami-rs/cami/blob/06cfc120812179e71a291a92b9c1034a792551eb/src/macros_c.rs#L135>.
///
/// A smaller benefit: Less duplication in `c_partial_eq` & `c_ord` macros: no need for an
/// (anonymous) filler closure.
///
/// This has to return a reference (so it can feed to comparison operators in general, even if the
/// wrapped/forwarded field is not [Copy]), hence "_ref" in its name.
#[doc(hidden)]
#[inline]
pub const fn always_equal_ref<T>(_: &T) -> &() {
    &()
}

#[macro_export]
macro_rules! cami_partial_eq {
    (
     $( <[
            $( $generic_left:tt )+
         ]>
     )?
     { $( $struct_path_and_generic_right:tt
        )+
     }
     $(where { $( $where:tt )* }
      )?

     ( $locality: expr
     )
    [
        $( $local:tt )*
    ]
    [
        $( $non_local:tt )*
    ]
    $(
    [
        $( $cami:tt )*
    ]
    )?
    ) => {
        $crate::cami_partial_eq_full_squares! {
            $( <[ $( $generic_left )+
                ]>
             )?
            { $( $struct_path_and_generic_right )+
            }
            $( where { $( $where )* }
             )?

            ( $locality
            )

            [
                ($crate::always_equal_ref::<Self>),
                $( $local )*
            ]
            [
                ($crate::always_equal_ref::<Self>),
                $( $non_local )*
            ]
            $(
            [
                ($crate::always_equal_ref::<Self>),
                $( $cami )*
            ]
            )?
        }
    };
}

#[macro_export]
macro_rules! cami_partial_eq_full_squares {
    (
     $( <[
            $( $generic_left:tt )+
         ]>
     )?
     { $( $struct_path_and_generic_right:tt
        )+
     }
     $(where { $( $where:tt )* }
      )?

     ( $locality: expr
     )

     [
        $(
           $({$local_get_closure:expr}
            )?

           $(($local_ref_closure:expr)
            )?

           // This is necessary only to match fields/chains of fields that have the first/top level
           // field a numeric index to a tuple. (We can't match it with :literal, because then the
           // generated code fails to compile due to scope/context mixed in.)
           $(
            $( .
               $local_dotted:tt
               $( (
                   // This does NOT match "expressions" passed to functions. It's here ONLY to
                   // capture a pair of PARENS with NO parameters within.
                   //
                   // Why NO parameters? Because we don't parse/handle this, so we can't the values
                   // of parameters. If we transcribed them in this macro, they would get evaluated
                   // twice. Possibly moving non-Copy values in - so the 2nd replica could fail to
                   // compile. Even worse, the expressions could have side effects, which could be
                   // duplicated.
                   $( to-match-optional-outer-parens: $local_dotted_then_within_parens:vis )?
                  )
               )?
            )+
           )?

           $(
               $local_ident:ident
               $( (
                   // This does NOT match "expressions" passed to functions. It's here ONLY to
                   // capture a pair of PARENS with NO parameters within.
                   $( to-match-optional-outer-parens: $local_ident_then_within_parens:vis )?
                  )
               )?
               // Same as "local_dotted" part above.
               $( .
                  $( $local_ident_then_dotted:tt )?
                  $( (
                       // This does NOT match "expressions" passed to functions. It's here ONLY to
                       // capture a pair of PARENS with NO parameters within.
                       $( to-match-optional-outer-parens: $local_ident_then_dotted_then_within_parens:vis )?
                     )
                  )?
               )*
           )?
        ),*
     ]
     [
        $(
           $({$non_local_get_closure:expr}
            )?

           $(($non_local_ref_closure:expr)
            )?

           $(
            $( .
               $non_local_dotted:tt
               $( (
                   $( to-match-optional-outer-parens: $non_local_dotted_then_within_parens:vis )?
                  )
               )?
            )+
           )?

           $(
               $non_local_ident:ident
               $( (
                   $( to-match-optional-outer-parens: $non_local_ident_then_within_parens:vis )?
                  )
               )?
               $( .
                  $( $non_local_ident_then_dotted:tt )?
                  $( (
                       $( to-match-optional-outer-parens: $non_local_ident_then_dotted_then_within_parens:vis )?
                     )
                  )?
               )*
           )?
        ),*
     ]
     $(
     [
        $(
           $({$cami_get_closure:expr}
            )?

           $(($cami_ref_closure:expr)
            )?

           $(
            $( .
               $cami_dotted:tt
               $( (
                   $( to-match-optional-outer-parens: $cami_dotted_then_within_parens:vis )?
                  )
               )?
            )+
           )?

           $(
               $cami_ident:ident
               $( (
                   $( to-match-optional-outer-parens: $cami_ident_then_within_parens:vis )?
                  )
               )?
               $( .
                  $( $cami_ident_then_dotted:tt )?
                  $( (
                       $( to-match-optional-outer-parens: $cami_ident_then_dotted_then_within_parens:vis )?
                     )
                  )?
               )*
           )?
        ),*
     ]
     )?
    ) => {
        /* */
        // const _: () = { panic!( stringify!(
        impl $(< $( $generic_left )+ >)?
        cami::CamiPartialEq for
        $( $struct_path_and_generic_right )+
        $(where $( $where )* )?
        {
            const LOCALITY: $crate::Locality = $locality;

            #[must_use]
            #[inline]
            fn eq_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_local();
                true
                $(
                    $(&& {
                        let getter: fn(&Self) -> _ = $local_get_closure;
                        getter(self) == getter(other)
                      }
                     )?

                    $(&& {
                        let getter: for<'a> fn(&'a Self) -> &'a _ = $local_ref_closure;
                        getter(self) == getter(other)
                      }
                     )?

                    $(&& self  $( .
                                  $local_dotted
                                  $( (
                                       $( $local_dotted_then_within_parens )?
                                     )
                                   )?
                                )+
                        ==
                         other $( .
                                  $local_dotted
                                  $( (
                                       $( $local_dotted_then_within_parens )?
                                     )
                                   )?
                                )+
                    )?

                    $(&& self  .
                               $local_ident
                               $( (
                                    $( $local_ident_then_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $local_ident_then_dotted )?
                                  $( (
                                       $( $local_ident_then_dotted_then_within_parens )?
                                     )
                                   )?
                                )*
                        ==
                         other  .
                               $local_ident
                               $( (
                                    $( $local_ident_then_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $local_ident_then_dotted )?
                                  $( (
                                       $( $local_ident_then_dotted_then_within_parens )?
                                     )
                                   )?
                                )*
                    )?
                )*
                $(
                $(
                    $(&& {
                        let getter: fn(&Self) -> _ = $cami_get_closure;
                        getter(self).eq_local( getter(other) )
                      }
                     )?

                    $(&& {
                        let getter: for<'a> fn(&'a Self) -> &'a _ = $cami_ref_closure;
                        getter(self).eq_local( getter(other) )
                      }
                     )?

                    $(&& self  $( .
                                  $cami_dotted
                                  $( (
                                       $( $cami_dotted_then_within_parens )?
                                     )
                                   )?
                                )+
                        .eq_local( &
                         other $( .
                                  $cami_dotted
                                  $( (
                                       $( $cami_dotted_then_within_parens )?
                                     )
                                   )?
                                )+
                        )
                    )?

                    $(&& self  .
                               $cami_ident
                               $( (
                                    $( $cami_ident_then_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $cami_ident_then_dotted )?
                                  $( (
                                       $( $cami_ident_then_dotted_then_within_parens )?
                                     )
                                   )?
                                )*
                        .eq_local( &
                         other  .
                               $cami_ident
                               $( (
                                    $( $cami_ident_then_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $cami_ident_then_dotted )?
                                  $( (
                                       $( $cami_ident_then_dotted_then_within_parens )?
                                     )
                                   )?
                                )*
                        )
                    )?
                )*
                )?
            }

            #[must_use]
            #[inline]
            fn eq_non_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_non_local();
                true
                $(
                    $(&& {
                        let getter: fn(&Self) -> _ = $non_local_get_closure;
                        getter(self) == getter(other)
                      }
                     )?

                    $(&& {
                        let getter: for<'a> fn(&'a Self) -> &'a _ = $non_local_ref_closure;
                        getter(self) == getter(other)
                      }
                     )?

                    $(&& self  $( .
                                  $non_local_dotted
                                  $( (
                                       $( $non_local_dotted_then_within_parens )?
                                     )
                                   )?
                                )+
                        ==
                         other $( .
                                  $non_local_dotted
                                  $( (
                                       $( $non_local_dotted_then_within_parens )?
                                     )
                                   )?
                                )+
                    )?

                    $(&& self  .
                               $non_local_ident
                               $( (
                                    $( $non_local_ident_then_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $non_local_ident_then_dotted )?
                                  $( (
                                       $( $non_local_ident_then_dotted_then_within_parens )?
                                     )
                                   )?
                                )*
                        ==
                         other  .
                               $non_local_ident
                               $( (
                                    $( $non_local_ident_then_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $non_local_ident_then_dotted )?
                                  $( (
                                       $( $non_local_ident_then_dotted_then_within_parens )?
                                     )
                                   )?
                                )*
                    )?
                )*
                $(
                $(
                    $(&& {
                        let getter: fn(&Self) -> _ = $cami_get_closure;
                        getter(self).eq_non_local( getter(other) )
                      }
                     )?

                    $(&& {
                        let getter: for<'a> fn(&'a Self) -> &'a _ = $cami_ref_closure;
                        getter(self).eq_non_local( getter(other) )
                      }
                     )?

                    $(&& self  $( .
                                  $cami_dotted
                                  $( (
                                       $( $cami_dotted_then_within_parens )?
                                     )
                                   )?
                                )+
                        .eq_non_local( &
                         other $( .
                                  $cami_dotted
                                  $( (
                                       $( $cami_dotted_then_within_parens )?
                                     )
                                   )?
                                )+
                        )
                    )?

                    $(&& self  .
                               $cami_ident
                               $( (
                                    $( $cami_ident_then_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $cami_ident_then_dotted )?
                                  $( (
                                       $( $cami_ident_then_dotted_then_within_parens )?
                                     )
                                   )?
                                )*
                        .eq_non_local( &
                         other  .
                               $cami_ident
                               $( (
                                    $( $cami_ident_then_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $cami_ident_then_dotted )?
                                  $( (
                                       $( $cami_ident_then_dotted_then_within_parens )?
                                     )
                                   )?
                                )*
                        )
                    )?
                )*
                )?
            }
        }
        /* */
        // )) };
    };
}

/// Like [c_partial_eq], but for [cami::CamiOrd].
#[macro_export]
macro_rules! cami_ord {
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_path:path
     $(>$($generic_right:tt),+<)?

     $({
       // The name of the only (wrapped) field, or 0 if tuple, for example if the struct has been
       // defined by `c_wrap!` or `c_wrap_tuple!`.` Otherwise $t is empty.
       $t:tt
     })?

     $(where $($left:ty : $right:tt),+)?
     // Documentation of [c_partial_eq] applies, but replace "_eq_" with "_cmp_" .
     [
        $(
           $(
            $local_ident:ident
            $(. $($local_ident_ident:ident)? $($local_ident_idx:literal)?
             )*)?

           $(
            $local_idx:literal
            $(. $($local_idx_ident:ident)? $($local_idx_idx:literal)?
             )* )?

           $(($local_cmp_closure:expr))?
           $({$local_get_closure:expr})?
        ),*
     ]
     [
        $(
           $(
            $non_local_ident:ident
            $(. $($non_local_ident_ident:ident)? $($non_local_ident_idx:literal)?
             )*)?

           $(
            $non_local_idx:literal
            $(. $($non_local_idx_ident:ident)? $($non_local_idx_idx:literal)?
             )* )?

           $(($non_local_cmp_closure:expr))?
           $({$non_local_get_closure:expr})?
        ),*
     ]
    ) => {
        impl $(<$($generic_left $(: $bound)?)+>)?
        cami::CamiPartialOrd for $struct_path $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            #[must_use]
            #[inline]
            fn partial_cmp_local(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                use cami::CamiPartialEq;
                Self::LOCALITY.debug_reachable_for_local();
                let this = &self;
                $( let this = &this.$t;
                   let other = &other.$t;
                )?
                let result = ::core::option::Option::None;
                // LLVM should be able to optimize away the first comparison of
                // result==::core::cmp::Ordering::Equal
                $(
                    if matches!(result, ::core::option::Option::Some(::core::cmp::Ordering::Less | ::core::cmp::Ordering::Greater)) {
                        return result;
                    }
                    $(let result =
                         ::core::option::Option::Some(this.$local_ident
                        $(.$($local_ident_ident)? $($local_ident_idx)?
                         )* .cmp(
                         &other.$local_ident
                        $(.$($local_ident_ident)? $($local_ident_idx)?
                         )*
                        ));
                    )?
                    $(let result =
                         ::core::option::Option::Some(this.$local_idx
                        $(.$($local_idx_ident)? $($local_idx_idx)?
                         )* .cmp(
                         &other.$local_idx
                        $(.$($local_idx_ident)? $($local_idx_idx)?
                         )*
                        ));
                    )?
                    $(let result =
                        ::core::option::Option::Some($local_cmp_closure(&this, &other));
                    )?
                    $(let result =
                        ::core::option::Option::Some($local_get_closure(&this).cmp(&$local_get_closure(&other)));
                    )?
                )*
                result
            }
            #[must_use]
            #[inline]
            fn partial_cmp_non_local(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                use cami::CamiPartialEq;
                Self::LOCALITY.debug_reachable_for_non_local();
                let this = &self;
                $( let this = &this.$t;
                   let other = &other.$t;
                )?
                let result = ::core::option::Option::None;
                // LLVM should be able to optimize away the first comparison of
                // result==::core::cmp::Ordering::Equal
                $(
                    if matches!(result, ::core::option::Option::Some(::core::cmp::Ordering::Less | ::core::cmp::Ordering::Greater)) {
                        return result;
                    }
                    $(let result =
                         ::core::option::Option::Some(this.$non_local_ident
                        $(.$($non_local_ident_ident)? $($non_local_ident_idx)?
                         )* .cmp(
                         &other.$non_local_ident
                        $(.$($non_local_ident_ident)? $($non_local_ident_idx)?
                         )*
                        ));
                    )?
                    $(let result =
                         ::core::option::Option::Some(this.$non_local_idx
                        $(.$($non_local_idx_ident)? $($non_local_idx_idx)?
                         )* .cmp(
                         &other.$non_local_idx
                        $(.$($non_local_idx_ident)? $($non_local_idx_idx)?
                         )*
                        ));
                    )?
                    $(let result =
                        ::core::option::Option::Some($non_local_cmp_closure(&this, &other));
                    )?
                    $(let result =
                        ::core::option::Option::Some($non_local_get_closure(&this).cmp(&$non_local_get_closure(&other)));
                    )?
                )*
                result
            }
            // @TODO
            /*
            #[must_use]
            #[inline]
            fn lt_local(&self, other: &Self) -> bool {
                self.len() < other.len()
            }
            #[must_use]
            #[inline]
            fn lt_non_local(&self, other: &Self) -> bool {
                self < other
            }

            #[must_use]
            #[inline]
            fn le_local(&self, other: &Self) -> bool {
                self.len() <= other.len()
            }
            #[must_use]
            #[inline]
            fn le_non_local(&self, other: &Self) -> bool {
                self <= other
            }

            #[must_use]
            #[inline]
            fn gt_local(&self, other: &Self) -> bool {
                self.len() > other.len()
            }
            #[must_use]
            #[inline]
            fn gt_non_local(&self, other: &Self) -> bool {
                self > other
            }

            #[must_use]
            #[inline]
            fn ge_local(&self, other: &Self) -> bool {
                self.len() >= other.len()
            }
            #[must_use]
            #[inline]
            fn ge_non_local(&self, other: &Self) -> bool {
                self >= other
            }
            */
        }

        impl $(<$($generic_left $(: $bound)?)+>)?
        cami::CamiOrd for $struct_path $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            #[must_use]
            #[inline]
            fn cmp_local(&self, other: &Self) -> ::core::cmp::Ordering {
                use cami::CamiPartialEq;
                Self::LOCALITY.debug_reachable_for_local();
                let this = &self;
                $( let this = &this.$t;
                   let other = &other.$t;
                )?
                let result = ::core::cmp::Ordering::Equal;
                // LLVM should be able to optimize away the first comparison of
                // result==::core::cmp::Ordering::Equal
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    $(let result =
                         this.$local_ident
                        $(.$($local_ident_ident)? $($local_ident_idx)?
                         )* .cmp(
                         &other.$local_ident
                        $(.$($local_ident_ident)? $($local_ident_idx)?
                         )*
                        );
                    )?
                    $(let result =
                         this.$local_idx
                        $(.$($local_idx_ident)? $($local_idx_idx)?
                         )* .cmp(
                         &other.$local_idx
                        $(.$($local_idx_ident)? $($local_idx_idx)?
                         )*
                        );
                    )?
                    $(let result =
                        $local_cmp_closure(&this, &other);
                    )?
                    $(let result =
                        $local_get_closure(&this).cmp(&$local_get_closure(&other));
                    )?
                )*
                result
            }

            #[must_use]
            #[inline]
            fn cmp_non_local(&self, other: &Self) -> ::core::cmp::Ordering {
                use cami::CamiPartialEq;
                Self::LOCALITY.debug_reachable_for_non_local();
                let this = &self;
                $( let this = &this.$t;
                   let other = &other.$t;
                )?
                let result = ::core::cmp::Ordering::Equal;
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    $(let result =
                         this.$non_local_ident
                        $(.$($non_local_ident_ident)? $($non_local_ident_idx)?
                         )* .cmp(
                         &other.$non_local_ident
                        $(.$($non_local_ident_ident)? $($non_local_ident_idx)?
                         )*
                        );
                    )?
                    $(let result =
                         this.$non_local_idx
                        $(.$($non_local_idx_ident)? $($non_local_idx_idx)?
                         )* .cmp(
                         &other.$non_local_idx
                        $(.$($non_local_idx_ident)? $($non_local_idx_idx)?
                         )*
                        );
                    )?
                    $(let result =
                        $non_local_cmp_closure(&this, &other);
                    )?
                    $(let result =
                        $non_local_get_closure(&this).cmp(&$non_local_get_closure(&other));
                    )?
                )*
                result
            }
        }
    };
}

/// NOT a part of public API. Internal only.
#[macro_export]
macro_rules! pure_local_c_partial_eq {
    ($T:ident) => {
        impl cami::CamiPartialEq for $T {
            const LOCALITY: $crate::Locality = $crate::Locality::PureLocal;

            #[must_use]
            #[inline]
            fn eq_local(&self, other: &Self) -> bool {
                self == other
            }
            #[must_use]
            #[inline]
            fn eq_non_local(&self, other: &Self) -> bool {
                ::cami_helpers::debug_fail_unreachable_for_non_local();
                self == other
            }
        }
    };
}

/// NOT a part of public API. Internal only.
#[macro_export]
macro_rules! pure_local_c_ord {
    ($T:ident) => {
        impl cami::CamiPartialOrd for $T {
            #[must_use]
            #[inline]
            fn partial_cmp_local(
                &self,
                other: &Self,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                self.partial_cmp(other)
            }
            #[must_use]
            #[inline]
            fn partial_cmp_non_local(
                &self,
                other: &Self,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::cami_helpers::debug_fail_unreachable_for_non_local();
                self.partial_cmp(other)
            }
        }

        impl cami::CamiOrd for $T {
            #[must_use]
            #[inline]
            fn cmp_local(&self, other: &Self) -> core::cmp::Ordering {
                self.cmp(other)
            }

            #[must_use]
            #[inline]
            fn cmp_non_local(&self, other: &Self) -> core::cmp::Ordering {
                ::cami_helpers::debug_fail_unreachable_for_non_local();
                self.cmp(other)
            }
        }
    };
}
