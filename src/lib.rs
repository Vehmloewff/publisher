use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct Publisher<T> {
    cell: UnsafeCell<Vec<T>>,
}

impl<T> Publisher<T> {
    pub fn new() -> Publisher<T> {
        Publisher {
            cell: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn publish(&self, item: T) {
        let pointer = self.cell.get();

        // SAFTEY: ensure that nobody is allowed to remove diagnostics from the stack until the references expire
        let vec = unsafe { &mut *pointer };

        vec.push(item)
    }

    pub fn items(self) -> Vec<T> {
        self.cell.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn inner(publisher: &Publisher<String>) {
            publisher.publish("hello".into())
        }

        let publisher = Publisher::new();
        inner(&publisher);

        assert_eq!(publisher.items(), vec![String::from("hello")])
    }
}
