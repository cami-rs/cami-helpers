use camigo_helpers::{cami_ord, cami_partial_eq, cami_wrap_struct, cami_wrap_tuple, Locality};

#[test]
fn main() {}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct A {
    x: i32,
    v: Vec<i32>,
}

cami_wrap_struct! {
    _CaWrap2 <A> {
        pub t : Vec<A>
    }
}

cami_wrap_struct! { CaWrapA1 {t : A }}
cami_partial_eq! {
    CaWrapA1 {
        Locality::Both => t
    }
    [ (|this: &A, other: &A| this.x==other.x) ]
    [.v]
}
cami_ord! {
    CaWrapA1 { t }
    [ {|a: &A| a.x} ]
    [v]
}

cami_wrap_tuple! { _CaTupleGen1 <T> (pub T) where T: Clone}

cami_wrap_tuple! { CaTupleA2 (A) }
fn get_v<'a>(wrap: &'a A) -> &'a Vec<i32> {
    &wrap.v
}
cami_partial_eq! {
    <'a>
    CaTupleA2 {
        Locality::Both => 0
    }
    [ {|obj: &A| obj.x}
    ]
    // @TODO: We can't specify return lifetimes here:
    //
    // [{ |obj: &'l A| -> &'l Vec<i32> {&obj.v} }]
    //
    // Hence a separate function:
    [ {get_v} ]
    []
}
cami_ord! {
    CaTupleA2 { 0 }
    [( |this: &A, other: &A| this.x.cmp(&other.x) )]
    [v]
}
