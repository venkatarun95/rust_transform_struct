use transform_struct::transform_struct;

fn round_float(f: f64) -> u64 {
    f.round() as u64
}

fn round_float_pair(f: (f64, f64)) -> (u64, u64) {
    (f.0 as u64, f.1 as u64)
}

transform_struct!(
    #[derive(Clone,PartialEq)]
    pub struct TestStruct1 TestNewStruct1 {
        a: char,
        > {
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
    struct TestStruct2 TestNewStruct2 {
        pub a: char
        > {
            pub f:f64 => (round_float -> u64)
        }
    }
);

transform_struct!(
    pub struct TestStruct3 TestNewStruct3 {
        a: char,
        b: (u8, u64),
        > {
            f:f64 => (round_float -> u64),
        }
    }
);

transform_struct!(
    pub struct TestStruct4 TestNewStruct4 {
        > {
            f:f64 => (round_float -> u64)
        }
    }
);

transform_struct!(
    pub struct TestStruct5 TestNewStruct5 {
        > {
            f: (f64, f64) => (round_float_pair -> (u64, u64)),
        }
    }
);

transform_struct!(
    #[derive(Clone)]
    struct TestStruct6 TestNewStruct6 {
        pub a: u8
    }
);

transform_struct!(
    pub struct TestStruct7 TestNewStruct7 {
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
