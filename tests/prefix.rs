#[macro_use]
extern crate getset;

// For testing the prefix attribute
#[derive(Getters, Default)]
pub struct PrefixTest {
    #[getset(get = "pub", prefix = "custom_")]
    field1: usize,

    #[getset(get = "pub", prefix = "mock_")]
    field2: String,

    #[getset(get = "pub")]
    field3: bool,
}

impl PrefixTest {
    fn new() -> Self {
        PrefixTest {
            field1: 42,
            field2: "hello".to_string(),
            field3: true,
        }
    }
}

#[test]
fn test_custom_prefix() {
    let val = PrefixTest::new();
    assert_eq!(42, *val.custom_field1());
    assert_eq!("hello", val.mock_field2());
    assert_eq!(true, *val.field3());
}

