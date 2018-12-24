# baggie

---

[![Build Status](https://travis-ci.com/milesgranger/baggie.svg?branch=master)](https://travis-ci.com/milesgranger/baggie)
[![crates.io](http://meritbadge.herokuapp.com/baggie)](https://crates.io/crates/baggie)
[![Coverage Status](https://coveralls.io/repos/github/milesgranger/baggie/badge.svg?branch=master)](https://coveralls.io/github/milesgranger/baggie?branch=master)

`Baggie` is simple interface for storing any type of element in a `HashMap`. 
The crate has no dependencies, and is really just a helper around storing and 
fetching `Any`s from a `HashMap`. It has no unsafe code and free of any unwraps 
or similar misgivings.

The `Baggie` implements a subset of methods found in HashMap.

The downside of this crate is you must know the type of what you stored later on. 
Typically this shouldn't be a problem, as you could keep some metadata structure 
describing what types belong to what keys you've stored.

Sometimes you might need a tool like this, but most times you should be using an enum. :)

```rust
use baggie::Baggie;

let mut bag = Baggie::new();

// Insert any value type you wish...
bag.insert("key1", "Value1".to_owned());
bag.insert("key2", vec!["value", "2"]);
bag.insert("key3", 3);

// Get a reference
let val3 = bag.get::<i32, _>("key3");
assert_eq!(Some(&3), val3);

// Get a mutable reference
let val2: Option<&mut Vec<&str>> = bag.get_mut("key2");
match val2 {
    Some(v) => *v = vec!["new", "value", "2"],
    None => panic!()
}
let val2: &mut Vec<&str> = bag.get_mut("key2").unwrap();
assert_eq!(val2, &mut vec!["new", "value", "2"]);
```