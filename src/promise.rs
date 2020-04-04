use std::ffi::c_void;
use std::mem;
use std::ptr::NonNull;

use crate::client::Client;
use crate::object::Object;
use crate::sys::{self, TmiClient, TmiObject, TmiPromise};

unsafe extern "C" fn and_then(
    client: *mut TmiClient,
    obj: *mut TmiObject,
    userdata: *mut c_void,
) -> *mut TmiPromise {
    let cli_userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, cli_userdata).expect("and_then called with NULL");
    let mut object = Object::new(false, obj).expect("and_then called with NULL");
    let userdata = Box::from_raw(userdata as *mut AndThenUserdata);
    let promise = (userdata.callback)(&mut client, &mut object);
    promise.inner.as_ptr()
}

unsafe extern "C" fn or_else(client: *mut TmiClient, obj: *mut TmiObject, userdata: *mut c_void) {
    let cli_userdata = sys::tmi_userdata(client);
    let mut client = Client::with_userdata(client, cli_userdata).expect("or_else called with NULL");
    let mut object = Object::new(false, obj).expect("or_else called with NULL");
    let userdata = Box::from_raw(userdata as *mut OrElseUserdata);
    (userdata.callback)(&mut client, &mut object);
}

unsafe extern "C" fn drop_promise(_: *mut TmiClient, _: *mut TmiObject, _: *mut c_void) {}

type AndThenCallback = Box<dyn Fn(&mut Client, &mut Object) -> Promise>;
type OrElseCallback = Box<dyn Fn(&mut Client, &mut Object)>;

struct AndThenUserdata {
    callback: AndThenCallback,
}

struct OrElseUserdata {
    callback: OrElseCallback,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Promise {
    inner: NonNull<TmiPromise>,
}

impl Promise {
    pub(crate) fn new(ptr: *mut TmiPromise) -> Option<Self> {
        let inner = NonNull::new(ptr)?;
        Some(Promise { inner })
    }

    pub fn and_then<F>(self, func: F) -> Promise
    where
        F: Fn(&mut Client, &mut Object) -> Promise + 'static,
    {
        let inner = self.inner.as_ptr();
        let userdata = Box::new(AndThenUserdata {
            callback: Box::new(func),
        });
        unsafe {
            sys::tmi_promise_set_userdata(inner, Box::into_raw(userdata) as *mut c_void);

            let promise = sys::tmi_promise_and_then(inner, Some(and_then));
            Promise::new(promise).expect("tmi_promise_and_then returned NULL")
        }
    }

    pub fn or_else<F>(self, func: F)
    where
        F: Fn(&mut Client, &mut Object) + 'static,
    {
        let inner = self.inner.as_ptr();
        let userdata = Box::new(OrElseUserdata {
            callback: Box::new(func),
        });
        mem::forget(self);
        unsafe {
            sys::tmi_promise_set_userdata(inner, Box::into_raw(userdata) as *mut c_void);

            sys::tmi_promise_or_else(inner, Some(or_else));
        }
    }
}

impl Drop for Promise {
    fn drop(&mut self) {
        unsafe {
            sys::tmi_promise_or_else(self.inner.as_ptr(), Some(drop_promise));
        }
    }
}
