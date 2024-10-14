# Publisher

With an immutable reference, publish items to a store, then get them all at once.

```rust
fn inner(publisher: &Publisher<String>) {
    publisher.publish("hello".into());
}

let publisher = Publisher::new();
inner(&publisher);

assert_eq!(publisher.items(), vec![String::from("hello")])
```
