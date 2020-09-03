use transform_struct::transform_struct;

fn round_float(f: f64) -> u64 {
    f.round() as u64
}

fn round_float_pair(f: (f64, f64)) -> (u64, u64) {
    (f.0 as u64, f.1 as u64)
}

transform_struct!(
    /// Foo
    #[derive(Clone,PartialEq)]
    /// Bar
    pub struct TestStruct1
    /// Bar2
    pub struct TestNewStruct1 {
        /// Comment
        a: char,
        > {
            /// Comment
            f:f64 => (round_float -> u64)
        }
    }
);

#[test]
fn test_into() {
    let t = TestStruct1 {
        a: 'a',
        f: 1.1,
    };

    let tn: TestNewStruct1 = t.into();
    assert_eq!(tn.a, 'a');
    assert_eq!(tn.f, 1)
}

transform_struct!(
    #[derive(Clone)]
    struct TestStruct2
    /// Foo
    struct TestNewStruct2 {
        pub a: char
        > {
            pub f:f64 => (round_float -> u64)
        }
    }
);

transform_struct!(
    pub struct TestStruct3
    #[derive(Clone)]
    struct TestNewStruct3 {
        a: char,
        b: (u8, u64),
        > {
            f:f64 => (round_float -> u64),
        }
    }
);

transform_struct!(
    pub struct TestStruct4
    struct TestNewStruct4 {
        > {
            f:f64 => (round_float -> u64)
        }
    }
);

transform_struct!(
    struct TestStruct5
    pub struct TestNewStruct5 {
        > {
            f: (f64, f64) => (round_float_pair -> (u64, u64)),
        }
    }
);

transform_struct!(
    /// Foo
    #[derive(Clone)]
    /// Bar
    struct TestStruct6
    struct TestNewStruct6 {
        /// Comment
        pub a: u8
    }
);

transform_struct!(
    pub struct TestStruct7
    pub struct TestNewStruct7 {
        a: u8,
    }
);

#[test]
fn test_derive() {
    let x = TestStruct1 { a : 'a', f : 1.2 };
    let _y = x.clone();

    let x = TestStruct6 { a : 0 };
    let _y = x.clone();
}
