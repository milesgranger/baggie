
extern crate baggie;

use baggie::Baggie;


#[test]
fn test_baggie_get() {

    let mut bag = Baggie::new();

    bag.insert("key1", "Value1".to_owned());
    bag.insert("key2", vec!["value", "2"]);
    bag.insert("key3", 3);

    let val: Option<&String> = bag.get("key1");
    assert_eq!(Some(&"Value1".to_owned()), val);

    let val = bag.get::<Vec<&str>, _>("key2");
    assert_eq!(Some(&vec!["value", "2"]), val);

    let val = bag.get::<i32, _>("key3");
    assert_eq!(Some(&3), val);

    // Arbitrary tests...
    assert_eq!(bag.len(), 3);  // len

    // keys()
    let mut keys = bag.keys().collect::<Vec<&&str>>();
    keys.sort();
    assert_eq!(keys, vec![&"key1", &"key2", &"key3"]);

    // contains_key()
    assert!(!bag.contains_key("key-does-not-exist"));

    // remove()
    assert!(bag.contains_key("key1"));
    bag.remove("key1");
    assert!(!bag.contains_key("key1"));

    // is_empty()
    assert!(!bag.is_empty());

    // clear()
    bag.clear();
    assert!(bag.is_empty());

    // debug
    println!("{:?}", &bag);
}


#[test]
fn test_baggie_get_mut() {
    let mut bag = Baggie::new();

    bag.insert("key1", "Value1".to_owned());
    bag.insert("key2", vec!["value", "2"]);
    bag.insert("key3", 3);

    let val: &mut String = bag.get_mut("key1").unwrap();
    assert_eq!(&mut "Value1".to_owned(), val);
    *val = "new_value".to_string();

    let val: &mut String = bag.get_mut("key1").unwrap();
    assert_eq!(&mut "new_value".to_owned(), val);
}
