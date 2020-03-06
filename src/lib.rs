//! Like `.unwrap()` or `.or_else()` for arbitrary patterns.
//!
//! I'm not so sure this is a great idea, but it's useful for prototyping at least!


/// When you're pretty sure something will match a pattern.
///
/// It simply expands to a two-branch match:
/// ```
/// sure!(val, pattern => result; otherwise);
/// // expands to
/// match val {
///     pattern => result,
///     _ => otherwise
/// }
/// ```
/// 
/// For example:
/// ```
/// sure!(val, Enum::Variant{field, name: Enum::Variant2(num)} => (field, num); return Err("oops"))
/// // expands to
/// match val {
///   Enum::Variant{field, name: Enum::Variant2(num)} => (field, num),
///   _ => return Err("oops")
/// }
/// ```
/// The else branch is optional and defaults to a panic:
/// ```
/// let v = Enum::Var2;
/// sure!(v, Enum::Var1(n) => n);
/// // Will panic with "Expected v to match pattern: Enum::Var1(n)"
/// ```
/// 
/// Asserting slice patterns can still feel redundant, so there are some affordances for that.
/// If the pattern and result are the same, you can leave out the `=> result`.
/// If a let binding, pattern, and result are all the same, you can pull in the let.
/// ```
/// // These are all equivalent
/// let [a,b] = sure!(vec[..], [a,b] => [a,b]);
/// let [a,b] = sure!(vec[..], [a,b]);
/// sure!(let [a,b] = vec[..]);
/// ```
///
#[macro_export]
macro_rules! sure {
    // main form
    ($target:expr, $p:pat => $res:expr; $else:expr) => {
        match $target {
            $p => $res,
            _ => $else
        }
    };
    ($target:expr, $p:pat => $res:expr) => {
        sure!($target, $p => $res; panic!("Expected {} to match pattern: {}", stringify!($target), stringify!($p)))
    };

    // self-matching form
    // for tuple or slice patterns that also form the proper bindings
    ($target:expr, $pat:tt; $else:expr) => {
        sure!($target, $pat => $pat; $else)
    };
    ($target:expr, $pat:tt) => {
        sure!($target, $pat => $pat)
    };

    // inlined let, for self-matching bindings
    (let $pat:tt = $target:expr; $else:expr) => {
        let $pat = sure!($target, $pat; $else);
    };
    (let $pat:tt = $target:expr) => {
        let $pat = sure!($target, $pat);
    };
}
