/// A macro to generate a struct containing a list of fields that may be locked.
/// This allows for more intuitive concurrency, by abstracting away the lock-unlock logic
/// and potential lock mishaps.
///
/// # Examples
/// ```
/// struct HelloWorldObject {
///     message: &'static str
/// }
///
/// struct Output {
///     message: Option<&'static str>
/// }
///
/// use covalent::lock_data;
/// lock_data! {
///     HelloWorldData
///
///     hello_world: read HelloWorldObject,
///     output: write Output
/// }
///
/// fn create_locks() {
///     use std::sync::*;
///     use covalent::scene::{Event, EventHandler};
///
///     let hello_world = Arc::new(RwLock::new(HelloWorldObject { message: "Hello, world!" }));
///     let output = Arc::new(RwLock::new(Output { message: None }));
///
///     let data = Arc::new(RwLock::new(HelloWorldData {
///         hello_world: Arc::downgrade(&hello_world),
///         output: Arc::downgrade(&output)
///     }));
///
///     struct HelloWorldEvent {}
///     impl Event for HelloWorldEvent {}
///     let event_handler = Arc::new(RwLock::new(EventHandler::<HelloWorldEvent>::new()));
///
///     HelloWorldData::listen(&data, &event_handler, |event, hello_world, output| {
///         output.message = Some(hello_world.message);
///     });
/// }
/// ```
#[macro_export]
macro_rules! lock_data {
    ($struct_name:ident $($name:ident : $mutability:ident $data_type:ty),+) => {
        struct $struct_name {
        // Populate the fields of the struct.
            $(
                $name : std::sync::Weak<std::sync::RwLock<$data_type>>,
            )*
        }

        // Implement the listen function.
        impl $struct_name {
            fn listen<'a, E, F>(data: &std::sync::Arc<std::sync::RwLock<Self>>, handler: &std::sync::Arc<std::sync::RwLock<$crate::scene::EventHandler<E>>>, func: F)
                where E: $crate::scene::Event,
                      F: Fn(&E
                          $(
                          , lock_data!(@ generate parameter $mutability $data_type)
                          )*
                      ),
                      F: Send + Sync + 'static {
                let copy = std::sync::Arc::clone(data);
                let l = $crate::scene::Listener {
                    id: handler.write().unwrap().new_id(),
                    func: Box::new(move |event| {
                        let self_var = copy.read().unwrap();
                        lock_data!{ @ generate locks self_var, func, event, $($name, $mutability, $data_type),+ }
                    })
                };
                handler.write().unwrap().insert(l);
            }
        }
    };

    (@ generate parameter read $data_type:ty) => { &$data_type };
    (@ generate parameter write $data_type:ty) => { &mut $data_type };
    (@ generate parameter $mutability:ident $data_type:ty) => {
        compile_error!("This macro requires the mutability of a variable to be 'read' or 'write'")
    };

    (@ generate locks $s:ident, $f:ident, $e:ident,
        $name0:ident, $mutability0:ident, $data_type0: ty | $($mutability1:ident)* | $($guard1:ident)*
        ) => {

        if let Some(arc) = std::sync::Weak::upgrade(&$s.$name0) {
            match lock_data!( @ generate try mutability $mutability0 arc ) {
                Ok(lock_data!( @ generate mutability $mutability0 guard )) => {
                    $f($e, $(lock_data!( @ generate mutability $mutability1 &$guard1)),*, lock_data!( @ generate mutability $mutability0 &guard));
                    Ok(())
                },
                _ => Err($crate::scene::ListenError::LockUnavailable)
            }
        } else {
            Err($crate::scene::ListenError::RequirementDeleted)
        }
    };

    // A single-argument version
    (@ generate locks $s:ident, $f:ident, $e:ident,
        $name0:ident, $mutability0:ident, $data_type0: ty
        ) => {

        if let Some(arc) = std::sync::Weak::upgrade(&$s.$name0) {
            match lock_data!( @ generate try mutability $mutability0 arc ) {
                Ok(lock_data!( @ generate mutability $mutability0 guard )) => {
                    $f($e, lock_data!( @ generate mutability $mutability0 &guard));
                    Ok(())
                },
                _ => Err($crate::scene::ListenError::LockUnavailable)
            }
        } else {
            Err($crate::scene::ListenError::RequirementDeleted)
        }
    };

    // We need to capture the `self` variable due to Rust's variable hygiene.
    // It's in the variable `s` here.
    // We also need to capture the `func` that must be called, and the `event`.
    // We'll also need to pass a reference to the `guard` we made, for the same reason.
    (@ generate locks $s:ident, $f:ident, $e:ident,
        $name0:ident, $mutability0:ident, $data_type0: ty,
        $($tail:tt)* | $($mutability1:ident),* | $($guard1:ident),*
        ) => {

        if let Some(arc) = std::sync::Weak::upgrade(&$s.$name0) {
            match lock_data!( @ generate try mutability $mutability0 arc ) {
                Ok(lock_data!( @ generate mutability $mutability0 guard)) => {
                    lock_data!{ @ generate locks $s, $f, $e, $($tail)* | $($mutability1),*, $mutability0 | $($guard1),*, guard }
                },
                _ => Err($crate::scene::ListenError::LockUnavailable)
            }
        } else {
            Err($crate::scene::ListenError::RequirementDeleted)
        }
    };

    // Another copy of the above function that doesn't have the guard variables at the end.
    (@ generate locks $s:ident, $f:ident, $e:ident,
        $name0:ident, $mutability0:ident, $data_type0: ty,
        $($tail:tt)*
        ) => {

        if let Some(arc) = std::sync::Weak::upgrade(&$s.$name0) {
            match lock_data!( @ generate try mutability $mutability0 arc ) {
                Ok(lock_data!( @ generate mutability $mutability0 guard)) => {
                    lock_data!{ @ generate locks $s, $f, $e, $($tail)* | $mutability0 | guard }
                },
                _ => Err($crate::scene::ListenError::LockUnavailable)
            }
        } else {
            Err($crate::scene::ListenError::RequirementDeleted)
        }
    };

    (@ generate mutability read $thing:ident) => { $thing };
    (@ generate mutability write $thing:ident) => { mut $thing };
    (@ generate mutability read &$thing:ident) => { &$thing };
    (@ generate mutability write &$thing:ident) => { &mut $thing };
    (@ generate try mutability read $rwlock:ident) => { $rwlock.try_read() };
    (@ generate try mutability write $rwlock:ident) => { $rwlock.try_write() };
}
