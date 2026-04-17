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
}

impl Deref for PooledObject {
    type Target = MyObject;
    fn deref(&self) -> &Self::Target {
        // safe: while PooledObject is alive its `inner` is Some; Drop will take() it
        self.inner.as_ref().expect("pooled object moved")
    }
}

impl DerefMut for PooledObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().expect("pooled object moved")
    }
}

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

        println!(
            "Checked out objects: {} and {}",
            obj1.id(),
            obj2.id()
        );
    } // obj1 and obj2 go out of scope here, returning them to the pool

    let obj3 = pool.checkout().expect("pool should have object");
    println!("Checked out object: {}", obj3.id());

    {
        let objects = (1..=10).map(|_| pool.checkout()).collect::<Result<Vec<_>, _>>();
        match objects {
            Ok(objs) => println!("Checked out {} objects", objs.len()),
            Err(e) => println!("Error checking out objects: {:?}", e),
        }
    }
}

