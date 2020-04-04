use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::fmt::{self, Display};
use std::ptr::NonNull;

use crate::sys::{self, TmiObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConversionError(ObjectType, ObjectType);

impl Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "conversion from `Object` to a concrete type failed, got {:?}, expected {:?}",
            self.0, self.1
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Object,
    Number,
    Bool,
    Array,
    String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    owned: bool,
    inner: NonNull<TmiObject>,
}

impl Object {
    pub(crate) fn new(owned: bool, ptr: *mut TmiObject) -> Option<Self> {
        let inner = NonNull::new(ptr)?;
        Some(Object { owned, inner })
    }

    pub(crate) fn as_ptr(&self) -> *mut TmiObject {
        self.inner.as_ptr()
    }

    pub(crate) fn len(&self) -> Option<usize> {
        if self.is_array() {
            Some(
                usize::try_from(&self.get("length"))
                    .expect("the `length` property is something not a number"),
            )
        } else {
            None
        }
    }

    pub(crate) fn index(&self, idx: usize) -> Option<Object> {
        if self.is_array() {
            let object = unsafe { sys::tmi_object_index(self.as_ptr(), idx as _) };
            Some(Object::new(true, object).expect("tmi_object_index returned NULL"))
        } else {
            None
        }
    }

    pub fn get(&self, key: &str) -> Object {
        let key = CString::new(key).expect("invalid cstring passed into key");
        let object = unsafe { sys::tmi_object_get(self.as_ptr(), key.as_ptr()) };
        Object::new(true, object).expect("tmi_object_get returned NULL")
    }

    pub fn type_of(&self) -> ObjectType {
        if self.is_number() {
            ObjectType::Number
        } else if self.is_bool() {
            ObjectType::Bool
        } else if self.is_string() {
            ObjectType::String
        } else if self.is_array() {
            ObjectType::Array
        } else {
            ObjectType::Object
        }
    }

    pub fn is_object(&self) -> bool {
        unsafe { sys::tmi_object_is_object(self.as_ptr()) != 0 }
    }

    pub fn is_number(&self) -> bool {
        unsafe { sys::tmi_object_is_number(self.as_ptr()) != 0 }
    }

    pub fn is_bool(&self) -> bool {
        unsafe { sys::tmi_object_is_bool(self.as_ptr()) != 0 }
    }

    pub fn is_array(&self) -> bool {
        unsafe { sys::tmi_object_is_array(self.as_ptr()) != 0 }
    }

    pub fn is_string(&self) -> bool {
        unsafe { sys::tmi_object_is_string(self.as_ptr()) != 0 }
    }

    pub fn keys(&self) -> impl Iterator<Item = String> {
        let keys = unsafe { sys::tmi_object_get_properties(self.as_ptr()) };
        Vec::try_from(&Object::new(true, keys).expect("tmi_object_get_properties returned NULL"))
            .expect("tmi_object_get_properties returned something not an array")
            .into_iter()
            .map(|key| {
                String::try_from(&key)
                    .expect("tmi_object_get_properties returned something not an array of strings")
            })
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ptr = unsafe { sys::tmi_object_to_string(self.as_ptr()) };
        let cstring = unsafe { CStr::from_ptr(ptr) };
        let string = cstring.to_str().map_err(|_| fmt::Error)?;
        Display::fmt(&string, f)
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                sys::tmi_del_object(self.inner.as_ptr());
            }
        }
    }
}

impl<'a> TryFrom<&'a Object> for f64 {
    type Error = ConversionError;

    fn try_from(object: &'a Object) -> Result<Self, Self::Error> {
        if object.is_number() {
            unsafe { Ok(sys::tmi_object_to_number(object.as_ptr())) }
        } else {
            Err(ConversionError(object.type_of(), ObjectType::Number))
        }
    }
}

impl<'a> TryFrom<&'a Object> for usize {
    type Error = ConversionError;

    fn try_from(object: &'a Object) -> Result<Self, Self::Error> {
        f64::try_from(object).map(|float| float as usize)
    }
}

impl<'a> TryFrom<&'a Object> for bool {
    type Error = ConversionError;

    fn try_from(object: &'a Object) -> Result<Self, Self::Error> {
        if object.is_bool() {
            unsafe { Ok(sys::tmi_object_to_bool(object.as_ptr()) != 0) }
        } else {
            Err(ConversionError(object.type_of(), ObjectType::Bool))
        }
    }
}

impl<'a> TryFrom<&'a Object> for String {
    type Error = ConversionError;

    fn try_from(object: &'a Object) -> Result<Self, Self::Error> {
        if object.is_string() {
            // TODO: does this *const c_char need to be freed?
            let cstring = unsafe { CStr::from_ptr(sys::tmi_object_to_string(object.as_ptr())) };
            let string = cstring
                .to_str()
                .map_err(|_| ConversionError(object.type_of(), ObjectType::String))?
                .to_string();
            Ok(string)
        } else {
            Err(ConversionError(object.type_of(), ObjectType::String))
        }
    }
}

impl<'a> TryFrom<&'a Object> for Vec<Object> {
    type Error = ConversionError;

    fn try_from(object: &'a Object) -> Result<Self, Self::Error> {
        if object.is_array() {
            let len = object.len().unwrap();
            let mut vec = Vec::with_capacity(len);
            for i in 0..len {
                vec.push(object.index(i).unwrap());
            }
            Ok(vec)
        } else {
            Err(ConversionError(object.type_of(), ObjectType::Array))
        }
    }
}

impl TryFrom<Object> for f64 {
    type Error = ConversionError;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Self::try_from(&object)
    }
}

impl TryFrom<Object> for usize {
    type Error = ConversionError;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Self::try_from(&object)
    }
}

impl TryFrom<Object> for bool {
    type Error = ConversionError;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Self::try_from(&object)
    }
}

impl TryFrom<Object> for String {
    type Error = ConversionError;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Self::try_from(&object)
    }
}

impl TryFrom<Object> for Vec<Object> {
    type Error = ConversionError;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Self::try_from(&object)
    }
}
