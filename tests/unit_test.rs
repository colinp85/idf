use idf::{Idf, IdoKeyT, IdoItemType, IdoItem};

#[test]
fn test_set_item() {
    let mut idf = Idf::new();
    let mut item = IdoItem::new();
    item.m_type = IdoItemType::STRING;
    item.m_string = String::from("JOHN");

    idf.set_item(&42, item.clone());

    assert!(idf.contains(&42));
    assert_eq!(idf.size(), 1);

    if let Some(value) = idf.get_item(&42) {
        assert_eq!(value.get_type(), item.m_type);
        assert_eq!(value.m_string, item.m_string);

    } else {
        panic!("failed to find the item!");
    }
}

#[test]
fn test_set_string() {
    let mut d = Idf::new();

    let key: IdoKeyT = 100;

    let srcval:  String = String::from("BLAH");

    d.set_string(&key, srcval.clone());

    if let Some(value) = d.get_string(&key) {
            assert_eq!(value, srcval);
    }
    else {
        assert!(false);
    }

    assert_eq!(d.size(), 1);
}

#[test]
fn test_set_string_overwrite() {
    let mut idf = Idf::new();
    let key: IdoKeyT = 123;
    let val1 = String::from("test value 1");
    let val2 = String::from("test value 2");

    idf.set_string(&key, val1.clone());
    idf.set_string(&key, val2.clone());

    if let Some(value) = idf.get_string(&key) {
            assert_eq!(value, val2);
    }
    else {
        assert!(false);
    }
}

#[test]
fn test_set_empty_string() {
    let mut idf = Idf::new();
    let key: IdoKeyT = 123;
    let val = String::new(); // Empty string

    idf.set_string(&key, val.clone());

    if let Some(value) = idf.get_string(&key) {
            assert_eq!(value, val);
    }
    else {
        assert!(false);
    }
}

#[test]
fn test_set_integer() {
    let mut idf = Idf::new();
    idf.set_integer(&1, 42);

    assert!(idf.contains(&1));
    assert_eq!(idf.size(), 1);
    assert_eq!(idf.get_i32(&1).unwrap(), 42);
}

#[test]
fn test_set_f64() {
    let mut idf = Idf::new();
    idf.set_f64(&1, 3.14159);

    assert!(idf.contains(&1));
    assert_eq!(idf.size(), 1);
    assert_eq!(idf.get_f64(&1).unwrap(), 3.14159);
}

#[test]
fn test_set_integer_negative() {
    let mut idf = Idf::new();
    idf.set_integer(&1, -42);

    assert_eq!(idf.get_i32(&1).unwrap(), -42);
}

#[test]
fn test_set_integer_large_number() {
    let mut idf = Idf::new();
    idf.set_integer(&1, 1000000);

    assert_eq!(idf.get_i32(&1).unwrap(), 1000000);
}

#[test]
fn test_set_integer_zero() {
    let mut idf = Idf::new();
    idf.set_integer(&1, 0);

    assert_eq!(idf.get_i32(&1).unwrap(), 0);
}

#[test]
fn test_set_integer_update_existing_key() {
    let mut idf = Idf::new();
    idf.set_integer(&1, 42);
    idf.set_integer(&1, 24);

    assert_eq!(idf.get_i32(&1).unwrap(), 24);
}

#[test]
fn test_append_array_key_not_exist() {
    let data = Idf::new();

    let mut idf = Idf::new();

    let key: IdoKeyT = 1; // Replace with the desired key value
    idf.append_array(&key, data);

    assert!(idf.contains(&key));

    assert!(idf.is_type(&key, &IdoItemType::ARRAY));

    if let Some(value) = idf.get_item(&key) {
        assert_eq!(value.m_array.len(), 1);
    } else {
        panic!("Key not found in the map");
    }
}

#[test]
fn test_clear() {
    let mut idf = Idf::new();
    idf.set_string(&1, "value1".to_string());
    idf.set_string(&2, "value2".to_string());

    assert_eq!(idf.size(), 2);

    idf.clear();

    assert_eq!(idf.size(), 0);
}

#[test]
fn test_size() {
    let mut idf = Idf::new();
    idf.set_string(&1, "value1".to_string());
    idf.set_string(&2, "value2".to_string());

    assert_eq!(idf.size(), 2);
}

#[test]
fn test_update() {
    let mut ido1 = Idf::new();
    ido1.set_string(&1, "value1".to_string());
    ido1.set_string(&2, "value2".to_string());

    let mut ido2 = Idf::new();
    ido2.set_string(&2, "new_value2".to_string());
    ido2.set_string(&3, "value3".to_string());

    ido1.update(&ido2);

    assert_eq!(ido1.size(), 3);
    assert_eq!(ido1.get_string(&1).unwrap(), "value1");
    assert_eq!(ido1.get_string(&2).unwrap(), "new_value2");
    assert_eq!(ido1.get_string(&3).unwrap(), "value3");
}

#[test]
fn test_contains() {
    let mut idf = Idf::new();
    idf.set_integer(&1, 25);

    assert!(idf.contains(&1));
    assert!(!idf.contains(&2));
}

#[test]
fn test_is_type() {
    let mut idf = Idf::new();
    idf.set_integer(&1, 25);
    idf.set_string(&2, "value".to_string());

    assert!(idf.is_type(&1, &IdoItemType::INTEGER));
    assert!(idf.is_type(&2, &IdoItemType::STRING));
    assert!(!idf.is_type(&1, &IdoItemType::STRING));
    assert!(!idf.is_type(&2, &IdoItemType::INTEGER));
    assert!(!idf.is_type(&3, &IdoItemType::INTEGER));
}

#[test]
fn test_get_i64() {
    let mut idf = Idf::new();
    idf.set_integer(&42, 100);

    assert_eq!(idf.get_i64(&42), Some(100));

    idf.set_string(&43, String::from("invalid"));
    assert_eq!(idf.get_i64(&43), None);

    assert_eq!(idf.get_i64(&99), None);
}

#[test]
fn test_get_i32() {
    let mut idf = Idf::new();
    idf.set_integer(&42, 100);

    assert_eq!(idf.get_i32(&42), Some(100));

    idf.set_string(&43, String::from("invalid"));
    assert_eq!(idf.get_i32(&43), None);

    assert_eq!(idf.get_i32(&99), None);
}

#[test]
fn test_get_i16() {
    let mut idf = Idf::new();
    idf.set_integer(&42, 100);

    assert_eq!(idf.get_i16(&42), Some(100));

    idf.set_string(&43, String::from("invalid"));
    assert_eq!(idf.get_i16(&43), None);

    assert_eq!(idf.get_i16(&99), None);
}

#[test]
fn test_get_i8() {
    let mut idf = Idf::new();
    idf.set_integer(&42, 100);

    assert_eq!(idf.get_i8(&42), Some(100));

    idf.set_string(&43, String::from("invalid"));
    assert_eq!(idf.get_i8(&43), None);

    assert_eq!(idf.get_i8(&99), None);
}

#[test]
fn test_get_u64() {
    let mut idf = Idf::new();
    idf.set_integer(&42, 100);

    assert_eq!(idf.get_u64(&42), Some(100));

    idf.set_string(&43, String::from("invalid"));
    assert_eq!(idf.get_u64(&43), None);

    assert_eq!(idf.get_u64(&99), None);
}

#[test]
fn test_get_u32() {
    let mut idf = Idf::new();
    idf.set_integer(&42, 100);

    assert_eq!(idf.get_u32(&42), Some(100));

    idf.set_string(&43, String::from("invalid"));
    assert_eq!(idf.get_u32(&43), None);

    assert_eq!(idf.get_u32(&99), None);
}

#[test]
fn test_get_u16() {
    let mut idf = Idf::new();
    idf.set_integer(&42, 100);

    assert_eq!(idf.get_u16(&42), Some(100));

    idf.set_string(&43, String::from("invalid"));
    assert_eq!(idf.get_u16(&43), None);

    assert_eq!(idf.get_u16(&99), None);
}

#[test]
fn test_get_u8() {
    let mut idf = Idf::new();
    idf.set_integer(&42, 100);

    assert_eq!(idf.get_u8(&42), Some(100));

    idf.set_string(&43, String::from("invalid"));
    assert_eq!(idf.get_u8(&43), None);

    assert_eq!(idf.get_u8(&99), None);
}

#[test]
fn test_get_f64() {
    let mut idf = Idf::new();
    idf.set_f64(&42, 3.14);

    assert_eq!(idf.get_f64(&42), Some(3.14));

    idf.set_string(&43, String::from("invalid"));
    assert_eq!(idf.get_f64(&43), None);

    assert_eq!(idf.get_f64(&99), None);
}

#[test]
fn test_delete_item() {
    let mut idf = Idf::new();
    idf.set_string(&1, "value1".to_string());
    idf.set_string(&2, "value2".to_string());

    idf.delete_item(&1);

    assert_eq!(idf.get_string(&1), None);
    assert_eq!(idf.get_string(&2), Some("value2".to_string()));
}