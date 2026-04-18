// Object Pool pattern

use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

struct MyObject {
    id: u32,
}

impl MyObject {
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
enum PoolError {
    Empty,
}

struct Pool {
    available: Arc<Mutex<Vec<MyObject>>>,
    // For single threaded: <Rc<RefCell<Vec<MyObject>>>>

    // NOTE: Arc - Atomic Reference Counted
    // Thread-safe reference counting pointer
    // allows multiple ownership across threads
    // Object is deallocated when the last Arc pointing to it is dropped
    //
    // NOTE: Rc - Reference Counted
    // Non-thread-safe reference counting pointer
    // allows multiple ownership within a single thread
    // Object is deallocated when the last Rc pointing to it is dropped
    //
    // NOTE: Mutex - Mutual Exclusion
    // Provides interior mutability and thread safety
    // -> ensures exclusive access to the data.
}

struct PooledObject {
    inner: Option<MyObject>,
    pool_ref: Arc<Mutex<Vec<MyObject>>>,
    // For single threaded: <Rc<RefCell<Vec<MyObject>>>>
}

impl Drop for PooledObject {
    fn drop(&mut self) {
        if let Some(obj) = self.inner.take() {
            println!("Returning object {} to pool", obj.id);
            self.pool_ref.lock().unwrap().push(obj);
            // For single threaded: self.pool_ref.borrow_mut().push(obj);
        }
    }

    // NOTE: Drop Trait
    // allows you to specify code that runs when a value goes out of scope
    //
    // NOTE: if let
    // A convenient syntax for matching a single pattern while ignoring the rest
    //
    // NOTE: Option::take
    // Takes the value out of the Option, leaving None in its place
}

impl Deref for PooledObject {
    type Target = MyObject;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().expect("pooled object moved")
    }
}

impl DerefMut for PooledObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().expect("pooled object moved")
    }
}

// NOTE: Deref and DerefMut Traits
// customize the behavior of the dereference operator (*)
// Deref - for immutable references, allows you to treat PooledObject as if it were a MyObject
// DerefMut - for mutable references, allows you to modify the MyObject through a mutable reference to PooledObject

impl Pool {
    fn checkout(&self) -> Result<PooledObject, PoolError> {
        let mut items = self.available.lock().unwrap();
        // For single threaded: let mut items = self.available.borrow_mut();
        items
            .pop()
            .map(|obj| PooledObject {
                inner: Some(obj),
                pool_ref: Arc::clone(&self.available),
            })
            .ok_or(PoolError::Empty)
    }

    // NOTE: Interior Mutability
    // &self is an immutable reference
    // Mutex allows us to mutate the internal state of the pool
}

fn main() {
    let pool = Pool {
        available: Arc::new(Mutex::new(
            (1..=5).map(|id| MyObject { id }).collect::<Vec<_>>(),
        )),
    };

    {
        let obj1 = pool.checkout().expect("pool empty");
        let obj2 = pool.checkout().expect("pool empty");

        println!("Checked out objects: {} and {}", obj1.id(), obj2.id());
    } // obj1 and obj2 go out of scope here, returning them to the pool

    // NOTE: Deref coercion
    // access to MyObject.id() through PooledObject without explicit dereferencing
    // Possible because or Deref trait
    // Manual deref - (*obj1).id() or obj1.deref().id() 

    let obj3 = pool.checkout().expect("pool should have object");
    println!("Checked out object: {}", obj3.id());

    {
        let objects = (1..=10)
            .map(|_| pool.checkout())
            .collect::<Result<Vec<_>, _>>();
        match objects {
            Ok(objs) => println!("Checked out {} objects", objs.len()),
            Err(e) => println!("Error checking out objects: {:?}", e),
        }
        // NOTE: Collecting Results
        // collect::<Result<Vec<_>, _>>() - collects Result with Vec / Error
        // collect::<Vec<Result<T, E>>>() - collects Vec of Results
        // For more see `rustlings - iterators`
    }
}
