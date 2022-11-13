use hex_literal::hex;
use super::*;

#[test]
fn test() {
    macro_rules! case {
        ($v:expr, $exp:expr) => {{
            println!("{:?}", &$v);
            let buf = $v.clone().encode();
            println!("{}", hex::encode($exp.as_slice()));
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
        72 03 0805
        2c 7b 62 05 45 68656c6c6f 47 676f6f64627965
        2e 000f423f 62 05 46 7468616e6b73 4b 686f772061726520796f75
        ")
    );

    case!(
        Value::Tuple(vec![
            Value::Unit,
            Value::Bool(false),
            Value::Int(-7777777),
            Value::UInt(24393),
            Value::Float(50.0_f64.to_bits()),
            Value::String("Berylosft".to_owned()),
            Value::Bytes(b"(\x00)".to_vec()),
            Value::Option(Type::String, Box::new(None)),
            Value::Option(Type::Bool, Box::new(Some(Value::Bool(true)))),
            Value::Alias(TypePtr::Hash(hex!("fedcba98765432")), Box::new(Value::Bytes(b"\xff".to_vec()))),
            Value::Enum(TypePtr::from_u16(0x5f49), 5, Box::new(Value::Int(5))),
            Value::Enum(TypePtr::from_u16(0x00aa), 163, Box::new(Value::UInt(12))),
            Value::Type(Type::List(Box::new(Type::List(Box::new(Type::Struct(TypePtr::from_u16(0xfe50))))))),
            Value::TypePtr(TypePtr::Hash(hex!("fedcba98765432"))),
            Value::ObjectRef(ObjectRef { ot: 0x0123, oid: 0x0123456789abcdef }),
            Value::Option(Type::Tuple(vec![Type::Int, Type::Unit]), Box::new(Some(Value::Tuple(vec![Value::Int(9), Value::Unit])))),
        ]),
        hex!("
        8c 10
        00
        01
        1e 00ed5be1
        2d 5f49
        32 4049
        49 426572796c6f736674
        53 280029
        03 05
        04 01 02
        05 ff fedcba98765432 51 ff
        95 5f49 1a
        9c a3 00aa 2c 0c
        06 08 08 0d fe50
        07 ff fedcba98765432
        08 0123 0123456789abcdef
        04 0a 02 02 00 82 1c 12 00
        ")
    )
}
