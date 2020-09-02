# Transform struct

A simple macro that defines two `struct`s, a `BaseStruct` and a `NewStruct`.
They have fields with identical names but potentially different types. It
implements `From<BaseStruct>` for `NewStruct`, where fields with identical types
are moved and fields with different types are transformed using the given
function. Here is an example usage:

```
use transform_struct::transform_struct;

fn round_float(f: f64) -> u64{
    f.round() as u64
}

transform_struct!(
    #[derive(Clone,PartialEq)]
    pub struct TestStruct1 TestNewStruct1 {
        a: char,
        > {
            pub f:f64 => (round_float -> u64)
        }
    }
);

```
