use hex_literal::hex;
use super::*;

#[test]
fn test() {
    macro_rules! case {
        ($v:expr, $exp:expr) => {{
            println!("{:?}", &$v);
            let buf = $v.clone().encode();
            println!("{}", hex::encode(&buf));
            assert_eq!(&buf, $exp.as_slice());
            let v2 = Value::decode(&buf).unwrap();
            assert_eq!($v, v2);
        }};
    }

    case!(
        Value::Map((Type::UInt, Type::List(Box::new(Type::String))), vec![
            (Value::UInt(123), Value::List(Type::String, vec![
                Value::String("hello".to_owned()),
                Value::String("goodbye".to_owned()),
            ])),
            (Value::UInt(999999), Value::List(Type::String, vec![
                Value::String("thanks".to_owned()),
                Value::String("how are you".to_owned()),
            ])),
        ]),
        hex!("
        92 03 0805
        3c 7b 82 05 55 68656c6c6f 57 676f6f64627965
        3e 000f423f 82 05 56 7468616e6b73 5b 686f772061726520796f75
        ")
    );

    case!(
        Value::Struct(0x0123456789abcdef, vec![
            Value::Unit,
            Value::Bool(false),
            Value::Int(-7777777),
            Value::UInt(24393),
            Value::Float(50.0),
            Value::String("Berylosft".to_owned()),
            Value::Bytes(b"(\x00)".to_vec()),
            Value::Option(Type::String, Box::new(None)),
            Value::Option(Type::Bool, Box::new(Some(Value::Bool(true)))),
            Value::Alias(0x0123456789abcdef, Box::new(Value::Bytes(b"\xFF".to_vec()))),
            Value::Enum(0xfedcba9876543210, 5, Box::new(Value::Int(5))),
            Value::Enum(0xfedcba9876543210, 163, Box::new(Value::UInt(12))),
        ]),
        hex!("
        dc 0c 0123456789abcdef
        00
        10
        2e 00ed5be1
        3d 5f49
        40 4049000000000000
        59 426572796c6f736674
        63 280029
        70 05
        71 01 11
        a0 0123456789abcdef 61 ff
        b5 fedcba9876543210 2a
        bc a3 fedcba9876543210 3c 0c
        ")
    )
}
