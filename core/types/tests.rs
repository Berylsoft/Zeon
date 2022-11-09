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
            Value::ObjectRef(0x0123, 0x0123456789abcdef),
            Value::Option(Type::Tuple(vec![Type::Int, Type::Unit]), Box::new(Some(Value::Tuple(vec![Value::Int(9), Value::Unit])))),
        ]),
        hex!("
        ac 0f
        00
        10
        2e 00ed5be1
        3d 5f49
        40 4049000000000000
        59 426572796c6f736674
        63 280029
        70 05
        71 01 11
        b0 ff fedcba98765432 61 ff
        c5 5f49 2a
        cc a3 00aa 3c 0c
        e0 08 08 0d fe50
        f0 0123 0123456789abcdef
        71 0a 02 02 00 a2 2c 12 00
        ")
    )
}
