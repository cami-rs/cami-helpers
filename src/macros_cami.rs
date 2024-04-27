#[cfg(feature = "unsafe")]
use core::ops::DerefPure;
use core::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! cami_wrap_struct {
    // An INTERNAL rule
    (@
     [$( $($derived:path),+
       )?
     ]
     $struct_vis:vis
     $struct_name:ident
     // @TODO Apply generic_params + where to cami_partial_eq + cami_ord.
     $([ $($generic_params:tt)+ // N: const u8 = 1, T: Eq = ...
       ])?
     $( where { $( $where:tt )* } // T: Sized + Debug, [T; N]: ...
      )?
     {
       $field_vis:vis
       $t:ident
       : $T:ty
     }
    ) => {
        /// A zero cost (transparent) wrapper struct around a given type. For use with other macros
        /// from this crate.
        $(#[derive($($derived),+)])?
        #[repr(transparent)]
        $struct_vis struct $struct_name
        $(< $( $generic_params )+ >
         )?
        $( where $( $where )* )?
        {
            $field_vis $t: $T
        }
    };

    // The following prevents recursion on incorrect macro invocation
    (@
     $($tt:tt)+
    ) => {
        INCORRECT_MACRO_INVOCATION
        $($tt:tt)+
    };

    // NOT adding Clone/Debug/Eq/Ord/PartialEq/PartialOrd to $derived
    ([$($($derived:path),+)?]
     $($tt:tt)+
    ) => {
        cami_wrap_struct! {
            @
            [$($($derived),+)?]
            $($tt)+
        }
    };

    // Default the derived trait impls
    ($($tt:tt)+) => {
        cami_wrap_struct! {
            @
            [
            ::core::clone::Clone, ::core::fmt::Debug, ::core::cmp::Eq, ::core::cmp::Ord,
            ::core::cmp::PartialEq, ::core::cmp::PartialOrd
            ]
            $($tt)+
        }
    };
}

/// Like standard tuples, this accepts any (optional) "where" bound(s) AFTER the definition of the
/// tuple-wrapped fields (AFTER `(wrapped-type)` or (pub wripped-type)...). So that is different to
/// [cami_wrap_struct].
#[macro_export]
macro_rules! cami_wrap_tuple {
    // An INTERNAL rule
    (@
     [ $( $($derived:path),+
        )?
     ]
     $struct_vis:vis
     $struct_name:ident
     $([ $($generic_params:tt)+ // N: const u8 = 1, T: Eq = ...
       ])?
     (
     $field_vis:vis
     $T:ty
     )
     /// The curly braces after "where" are unnecessary here. But we do require them, so it's
     /// consistent with cami_wrap_struct.
     $( where { $( $where:tt )* } // T: Sized + Debug, [T; N]: ...
     )?
    ) => {
        /// A zero cost (transparent) wrapper struct around a given type. For use with other macros
        /// from this crate.
        $( #[derive( $( $derived),+ )]
         )?
        #[repr(transparent)]
        $struct_vis struct $struct_name
        $(< $( $generic_params )+ >
         )?
        (
            $field_vis $T
        )
        $( where $( $where )* )?
        ;
    };

    // The following prevents recursion on incorrect macro invocation
    (@
     $($tt:tt)+
    ) => {
        INCORRECT_MACRO_INVOCATION
        $($tt:tt)+
    };

    // NOT adding Clone/Debug/Eq/Ord/PartialEq/PartialOrd to $derived
    ([$($($derived:path),+)?]
     $($tt:tt)+
    ) => {
        cami_wrap_tuple! {
            @
            [$($($derived),+)?]
            $($tt)+
        }
    };

    // Default the derived trait impls
    ($($tt:tt)+) => {
        cami_wrap_tuple! {
            @
            [
            ::core::clone::Clone, ::core::fmt::Debug, ::core::cmp::Eq, ::core::cmp::Ord,
            ::core::cmp::PartialEq, ::core::cmp::PartialOrd
            ]
            $($tt)+
        }
    };
}

/// NOT a part of public API. Only for use by macro-generated code. Subject to change.
///
/// The main benefit: With this, we don't need to capture the wrapped type in `cami_partial_eq` &
/// `cami_ord when we apply those macros to a (`#[repr(transparent)]`) wrapper struct or tuple. See
/// also how we needed `$t_type:ty` (in commit `06cfc12`):
/// <https://github.com/peter-kehl/camigo/blob/06cfc120812179e71a291a92b9c1034a792551eb/src/macros_c.rs#L135>.
///
/// A smaller benefit: Less duplication in `c_partial_eq` & `c_ord` macros: no need for an
/// (anonymous) filler closure.
// This has to return a reference, hence "_ref" in its name.
#[doc(hidden)]
#[inline]
pub const fn always_equal_ref<T>(_instance: &T) -> &() {
    &()
}

#[macro_export]
macro_rules! cami_partial_eq {
    (
     $( (
         $( $generic_left:tt )+
        )
     )?
     [ $( $struct_path_and_generic_right:tt
        )+
     ]

     $(where { $( $where:tt )* }
      )?

     // $locality is NOT an ident, so that we allow (const-time) expressions.
     { $locality: expr
       // Only for 1-field wrapper types (newtype):
       //
       // The name of the only (wrapped) field, or 0 if tuple, for example if the struct has been
       // defined by `cami_wrap_struct!` or `cami_wrap_tuple!`.` Otherwise see the second input
       // pattern of this macro.
       => $t:tt
     }
     // The following two or three square pairs [] represent:
     // - local field(s),
     // - non-local field(s), and
     // - optional: field(s) that themselves implement [camigo::CamiPartialEq] ("camigo field(s)").
     //
     // Any of those two or three kinds of fields may be "deep fields".
     //
     // Within each square pair [], repeat any of the four parts (or three parts in case of "camigo"
     // field(s)"):
     // - `..._ident` for non-tuple structs, or
     // - `..._idx` for tuples, or
     // - (` ..._eq_closure`) for an equality closure - except for "camigo field(s)". Each closure
     //   must receive two parameters, for example `this` and `other`. Both parameters' type is a
     //   reference to the wrapped type (if you provided `$t`), or `Self` (if no `$t`). The closure
     //   compares the same chosen field in both references, and returns their equality (as `bool`).
     // - {` ..._get_closure`} for an accessor closure. Each closure must receive one parameter, for
     //   example `instance` or `obj`. That parameter's type is a reference to the wrapped type (if
     //   you provided `$t`), or `Self` (if no `$t`). The closure returns (a reference, or a copy
     //   of) a chosen field (or a value based on that field). Ideally return a reference to the
     //   field; beware of returning value of a (silent) `Copy` of large types.
    [
        $( $local:tt )*
    ]
    // @TODO Make optional:
    [
        $( $non_local:tt )*
    ]
    $(
    [
        $( $camigo:tt )*
    ]
    )?
    ) => {
        $crate::cami_partial_eq_full_squares! {
            $( ( $( $generic_left )+
               ))?
            [ $( $struct_path_and_generic_right )+
            ]
            $(where { $( $where )* }
             )?
            { $locality
              => $t
            }

            [
                // Injecting a constant-generating closure, which, composed by this macro, yields
                // true. Without this, handling empty square pair [] was extremely difficult because
                // of "ambiguity: multiple successful parses" (because we need a zero-or-more
                // repetitive block that can match empty content).
                {$crate::always_equal_ref},
                $( $local )*
            ]
            [
                {$crate::always_equal_ref},
                $( $non_local )*
            ]
            $(
            [
                {$crate::always_equal_ref},
                $( $camigo )*
            ]
            )?
        }
    };

    (
     $( (
         $( $generic_left:tt )+
        )
     )?
     [ $( $struct_path_and_generic_right:tt
        )+
     ]
     $(where { $( $where:tt )* }
      )?

     { $locality: expr
     }
    [
        $( $local:tt )*
    ]
    [
        $( $non_local:tt )*
    ]
    $(
    [
        $( $camigo:tt )*
    ]
    )?
    ) => {
        $crate::cami_partial_eq_full_squares! {
            $( ( $( $generic_left )+
               ))?
            [ $( $struct_path_and_generic_right )+
            ]
            $( where { $( $where )* }
             )?

            { $locality
            }

            [
                {$crate::always_equal_ref},
                $( $local )*
            ]
            [
                {$crate::always_equal_ref},
                $( $non_local )*
            ]
            $(
            [
                {$crate::always_equal_ref},
                $( $camigo )*
            ]
            )?
        }
    };
}

#[macro_export]
macro_rules! cami_partial_eq_full_squares {
    (
     $( (
         $( $generic_left:tt )+
        )
     )?
     [ $( $struct_path_and_generic_right:tt
        )+
     ]
     $(where { $( $where:tt )* }
      )?

     { $locality: expr
       $( => $t:tt )?
     }

     [
        $(
           $(($local_eq_closure:expr)
            )?

           $({$local_get_closure:expr}
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
                   $( $local_within_parens:tt )?
                  )
               )?
            )+
           )?

           $(
               $local_ident:ident
               $( (
                   // This does NOT match "expressions" passed to functions. It's here ONLY to
                   // capture a pair of PARENS with NO parameters within.
                   $( $local_after_ident_within_parens:tt )?
                  )
               )?
               // Same as "local_dotted" part above.
               $( .
                  $( $local_after_ident_dotted:tt )?
                  $( (
                       // This does NOT match "expressions" passed to functions. It's here ONLY to
                       // capture a pair of PARENS with NO parameters within.
                       $( $local_after_ident_dotted_within_parens:tt )?
                     )
                  )?
               )*
           )?
        ),*
     ]
     [
        $(
           $(($non_local_eq_closure:expr)
            )?

           $({$non_local_get_closure:expr}
            )?

           $(
            $( .
               $non_local_dotted:tt
               $( (
                   $( $non_local_within_parens:tt )?
                  )
               )?
            )+
           )?

           $(
               $non_local_ident:ident
               $( (
                   $( $non_local_after_ident_within_parens:tt )?
                  )
               )?
               $( .
                  $( $non_local_after_ident_dotted:tt )?
                  $( (
                       $( $non_local_after_ident_dotted_within_parens:tt )?
                     )
                  )?
               )*
           )?
        ),*
     ]
     $(
     [
        $(
           $({$camigo_get_closure:expr}
            )?

           $(
            $( .
               $camigo_dotted:tt
               $( (
                   $( $camigo_within_parens:tt )?
                  )
               )?
            )+
           )?

           $(
               $camigo_ident:ident
               $( (
                   $( $camigo_after_ident_within_parens:tt )?
                  )
               )?
               $( .
                  $( $camigo_after_ident_dotted:tt )?
                  $( (
                       $( $camigo_after_ident_dotted_within_parens:tt )?
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
        camigo::CamiPartialEq for
        $( $struct_path_and_generic_right )+
        $(where $( $where )* )?
        {
            const LOCALITY: $crate::Locality = $locality;

            fn eq_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_local();
                let this = self;
                $( let this = &this.$t;
                   let other = &other.$t;
                )?
                true
                $(
                    $(&& $local_eq_closure(&this, &other)
                     )?

                    $(&& {
                        let getter = $local_get_closure;
                        getter(this) == getter(other)
                      }
                     )?

                    $(&& this  $( .
                                  $local_dotted
                                  $( (
                                       $( $local_within_parens )?
                                     )
                                   )?
                                )+
                        ==
                         other $( .
                                  $local_dotted
                                  $( (
                                       $( $local_within_parens )?
                                     )
                                   )?
                                )+
                    )?

                    $(&& this  .
                               $local_ident
                               $( (
                                    $( $local_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $local_after_ident_dotted )?
                                  $( (
                                       $( $local_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        ==
                         other  .
                               $local_ident
                               $( (
                                    $( $local_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $local_after_ident_dotted )?
                                  $( (
                                       $( $local_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                    )?
                )*
                $(
                $(
                    $(&& {
                        let getter = $camigo_get_closure;
                        getter(this).eq_local( getter(other) )
                      }
                     )?

                    $(&& this  $( .
                                  $camigo_dotted
                                  $( (
                                       $( $camigo_within_parens )?
                                     )
                                   )?
                                )+
                        .eq_local( &
                         other $( .
                                  $camigo_dotted
                                  $( (
                                       $( $camigo_within_parens )?
                                     )
                                   )?
                                )+
                        )
                    )?

                    $(&& this  .
                               $camigo_ident
                               $( (
                                    $( $camigo_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $camigo_after_ident_dotted )?
                                  $( (
                                       $( $camigo_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        .eq_local( &
                         other  .
                               $camigo_ident
                               $( (
                                    $( $camigo_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $camigo_after_ident_dotted )?
                                  $( (
                                       $( $camigo_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        )
                    )?
                )*
                )?
            }

            fn eq_non_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_non_local();
                let this = self;
                $( let this = &this.$t;
                   let other = &other.$t;
                )?
                true
                $(
                    $(&& $non_local_eq_closure(&this, &other)
                     )?

                    $(&& {
                        let getter = $non_local_get_closure;
                        getter(this) == getter(other)
                      }
                     )?

                    $(&& this  $( .
                                  $non_local_dotted
                                  $( (
                                       $( $non_local_within_parens )?
                                     )
                                   )?
                                )+
                        ==
                         other $( .
                                  $non_local_dotted
                                  $( (
                                       $( $non_local_within_parens )?
                                     )
                                   )?
                                )+
                    )?

                    $(&& this  .
                               $non_local_ident
                               $( (
                                    $( $non_local_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $non_local_after_ident_dotted )?
                                  $( (
                                       $( $non_local_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        ==
                         other  .
                               $non_local_ident
                               $( (
                                    $( $non_local_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $non_local_after_ident_dotted )?
                                  $( (
                                       $( $non_local_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                    )?
                )*
                $(
                $(
                    $(&& {
                        let getter = $camigo_get_closure;
                        getter(this).eq_non_local( getter(other) )
                      }
                     )?

                    $(&& this  $( .
                                  $camigo_dotted
                                  $( (
                                       $( $camigo_within_parens )?
                                     )
                                   )?
                                )+
                        .eq_non_local( &
                         other $( .
                                  $camigo_dotted
                                  $( (
                                       $( $camigo_within_parens )?
                                     )
                                   )?
                                )+
                        )
                    )?

                    $(&& this  .
                               $camigo_ident
                               $( (
                                    $( $camigo_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $camigo_after_ident_dotted )?
                                  $( (
                                       $( $camigo_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        .eq_non_local( &
                         other  .
                               $camigo_ident
                               $( (
                                    $( $camigo_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $camigo_after_ident_dotted )?
                                  $( (
                                       $( $camigo_after_ident_dotted_within_parens )?
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

/// Like [c_partial_eq], but for [camigo::CamiOrd].
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
        camigo::CamiOrd for $struct_path $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            fn cmp_local(&self, other: &Self) -> ::core::cmp::Ordering {
                use camigo::CamiPartialEq;
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

            fn cmp_non_local(&self, other: &Self) -> ::core::cmp::Ordering {
                use camigo::CamiPartialEq;
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
        impl camigo::CamiPartialEq for $T {
            const LOCALITY: $crate::Locality = $crate::Locality::PureLocal;

            fn eq_local(&self, other: &Self) -> bool {
                self == other
            }
            fn eq_non_local(&self, other: &Self) -> bool {
                ::camigo_helpers::debug_fail_unreachable_for_non_local();
                self == other
            }
            fn eq_full(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}

/// NOT a part of public API. Internal only.
#[macro_export]
macro_rules! pure_local_c_ord {
    ($T:ident) => {
        impl camigo::CamiOrd for $T {
            fn cmp_local(&self, other: &Self) -> core::cmp::Ordering {
                self.cmp(other)
            }

            fn cmp_non_local(&self, other: &Self) -> core::cmp::Ordering {
                ::camigo_helpers::debug_fail_unreachable_for_non_local();
                self.cmp(other)
            }

            fn cmp_full(&self, other: &Self) -> core::cmp::Ordering {
                self.cmp(other)
            }
        }
    };
}

// @TODO
impl From<CaWrap> for &str {
    fn from(_value: CaWrap) -> Self {
        panic!()
    }
}
impl From<&str> for CaWrap {
    fn from(_value: &str) -> Self {
        panic!()
    }
}

cami_wrap_struct! {
    pub CaWrap {
        t : u8
    }
}

// @TODO
impl Deref for CaWrap {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        panic!()
    }
}
impl DerefMut for CaWrap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        panic!()
    }
}
#[cfg(feature = "unsafe")]
unsafe impl DerefPure for CaWrap {}

fn _into() {
    let _caw: CaWrap = "".into();
    let _caw: CaWrap = <&str>::into("");
}
fn _from() {
    let _caw = CaWrap::from("");
}

fn _deref(caw: &CaWrap) {
    let _ = caw.len();
}

cami_wrap_struct! { [Clone, Debug] _CaWrap3 [ T ] {t : T }}
cami_wrap_struct! { [Clone, Debug] _CaWrap4 [T:Sized  ] {t : T }}
cami_wrap_struct! {
    [Clone, Debug]
    _CaWrap5 [ T ]
    where {
        T: 'static
    } {
        t : T
    }
}
cami_wrap_struct! { pub CaWrapPub {pub t : u8}}
