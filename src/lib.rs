#![warn(missing_docs)]
//! [`Baggie`] is simple interface for storing any type of element in a [`HashMap`]. The crate
//! has no dependencies, and is really just a helper around storing and fetching [`Any`]s from
//! a [`HashMap`]. It has no `unsafe` code and free of any `unwrap`s or similar misgivings.
//!
//! The [`Baggie`] implements a subset of methods found in [`HashMap`].
//!
//! The downside of this crate is you _must_ know the type of what you stored later on. Typically
//! this shouldn't be a problem, as you could keep some _metadata_ structure describing what types
//! belong to what keys you've stored.
//!
//! _Sometimes_ you might need a tool like this, but _most times_ you should be using an [`enum`]. :)
//!
//! ## Example
//! ```
//! use baggie::Baggie;
//!
//! let mut bag = Baggie::new();
//!
//! // Insert any value type you wish...
//! bag.insert("key1", "Value1".to_owned());
//! bag.insert("key2", vec!["value", "2"]);
//! bag.insert("key3", 3);
//!
//! // Get a reference
//! let val3 = bag.get::<i32>("key3");
//! assert_eq!(Some(&3), val3);
//!
//! // Get a mutable reference
//! let val2: Option<&mut Vec<&str>> = bag.get_mut("key2");
//! match val2 {
//!     Some(v) => *v = vec!["new", "value", "2"],
//!     None => panic!()
//! }
//! let val2: &mut Vec<&str> = bag.get_mut("key2").unwrap();
//! assert_eq!(val2, &mut vec!["new", "value", "2"]);
//!
//! ```
//! [`enum`]: https://doc.rust-lang.org/book/enums.html
//! [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
//! [`Any`]: https://doc.rust-lang.org/std/any/trait.Any.html
//! [`Baggie`]: struct.Baggie.html


mod baggie;
pub use self::baggie::Baggie;
