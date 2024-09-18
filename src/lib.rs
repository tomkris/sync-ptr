//! # sync-ptr
//! Sync & Send wrappers for raw pointer's in rust.
//! To use add "use sync_ptr::*;" to your file,
//! then you should be able to call my_ptr.as_sync_const() among others on any raw pointer
//! to obtain a wrapped version of your raw pointer that is Sync/Send.
//!

use std::fmt::{Formatter, Pointer};
use std::ops::Deref;

///
/// Wrapped mutable raw pointer that is Send+Sync
///
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SyncMutPtr<T>(*mut T);

unsafe impl<T> Sync for SyncMutPtr<T> {}
unsafe impl<T> Send for SyncMutPtr<T> {}

impl<T> Clone for SyncMutPtr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        SyncMutPtr(self.0)
    }
}

impl <T> Copy for SyncMutPtr<T> {

}
impl<T> Pointer for SyncMutPtr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.0, f)
    }
}

impl<T> SyncMutPtr<T> {
    ///
    /// Makes `ptr` Send+Sync
    ///
    /// # Safety
    /// The `ptr` parameter must be able to handle being sent and used in other threads concurrently,
    /// or special care must be taken when using the wrapped `ptr` to not use it
    /// in any way in other threads.
    ///
    #[inline(always)]
    pub const unsafe fn new(ptr: *mut T) -> Self {
        Self(ptr)
    }

    ///
    /// Makes a Send+Sync null ptr.
    ///
    #[inline(always)]
    pub const fn null() -> Self {
        Self(std::ptr::null_mut())
    }

    ///
    /// Casts `ptr` to another data type while keeping it Send+Sync.
    ///
    #[inline(always)]
    pub const fn cast<Y>(&self) -> SyncMutPtr<Y> {
        SyncMutPtr(self.0.cast())
    }

    ///
    /// Returns inner `ptr` which is then no longer Send+Sync.
    ///
    #[inline(always)]
    pub const fn inner(&self) -> *mut T {
        self.0
    }

    ///
    /// Makes `ptr` immutable.
    ///
    #[inline(always)]
    pub const fn as_sync_const(&self) -> SyncConstPtr<T> {
        SyncConstPtr(self.0)
    }

    ///
    /// Makes `ptr` immutable and no longer Sync.
    ///
    #[inline(always)]
    pub const fn as_send_const(&self) -> SendConstPtr<T> {
        SendConstPtr(self.0)
    }

    ///
    /// This is equivalent to .clone() and does nothing.
    ///
    #[inline(always)]
    pub const fn as_sync_mut(&self) -> SyncMutPtr<T> {
        SyncMutPtr(self.0)
    }

    ///
    /// Makes `ptr` no longer Sync.
    ///
    #[inline(always)]
    pub const fn as_send_mut(&self) -> SendMutPtr<T> {
        SendMutPtr(self.0)
    }
}

impl<T> Deref for SyncMutPtr<T> {
    type Target = *mut T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Into<*mut T> for SyncMutPtr<T> {
    #[inline(always)]
    fn into(self) -> *mut T {
        self.inner()
    }
}

impl<T> Into<*const T> for SyncMutPtr<T> {
    #[inline(always)]
    fn into(self) -> *const T {
        self.inner()
    }
}

///
/// Wrapped const raw pointer that is Send+Sync
///
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SyncConstPtr<T>(*const T);

unsafe impl<T> Sync for SyncConstPtr<T> {}
unsafe impl<T> Send for SyncConstPtr<T> {}

impl<T> Clone for SyncConstPtr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        SyncConstPtr(self.0)
    }
}

impl <T> Copy for SyncConstPtr<T> {

}

impl<T> Pointer for SyncConstPtr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.0, f)
    }
}

impl<T> SyncConstPtr<T> {
    ///
    /// Makes `ptr` Send+Sync
    ///
    /// # Safety
    /// The `ptr` parameter must be able to handle being sent and used in other threads concurrently,
    /// or special care must be taken when using the wrapped `ptr` to not use it
    /// in any way in other threads.
    ///
    #[inline(always)]
    pub const unsafe fn new(ptr: *const T) -> Self {
        Self(ptr)
    }

    ///
    /// Makes a Send+Sync null ptr.
    ///
    #[inline(always)]
    pub const fn null() -> Self {
        Self(std::ptr::null())
    }

    ///
    /// Casts `ptr` to another data type while keeping it Send+Sync.
    ///
    #[inline(always)]
    pub const fn cast<Y>(&self) -> SyncConstPtr<Y> {
        SyncConstPtr(self.0.cast())
    }

    ///
    /// Returns inner `ptr` which is then no longer Send+Sync.
    ///
    #[inline(always)]
    pub const fn inner(&self) -> *const T {
        self.0
    }

    ///
    /// This is equivalent to .clone() and does nothing.
    ///
    #[inline(always)]
    pub const fn as_sync_const(&self) -> SyncConstPtr<T> {
        SyncConstPtr(self.0)
    }

    ///
    /// Makes this `ptr` no longer Sync.
    ///
    #[inline(always)]
    pub const fn as_send_const(&self) -> SendConstPtr<T> {
        SendConstPtr(self.0)
    }

    ///
    /// Makes this `ptr` mutable
    ///
    /// # Safety
    /// Writing to immutable data is UB.
    ///
    #[inline(always)]
    pub const fn as_sync_mut(&self) -> SyncMutPtr<T> {
        SyncMutPtr(self.0.cast_mut())
    }

    ///
    /// Makes this `ptr` mutable and no longer Sync.
    ///
    /// # Safety
    /// Writing to immutable data is UB.
    ///
    #[inline(always)]
    pub const fn as_send_mut(&self) -> SendMutPtr<T> {
        SendMutPtr(self.0.cast_mut())
    }
}

impl<T> Deref for SyncConstPtr<T> {
    type Target = *const T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Into<*const T> for SyncConstPtr<T> {
    #[inline(always)]
    fn into(self) -> *const T {
        self.inner()
    }
}

///
/// Wrapped mutable raw pointer that is Send but not Sync
///
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SendMutPtr<T>(*mut T);

unsafe impl<T> Send for SendMutPtr<T> {}

impl<T> Clone for SendMutPtr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        SendMutPtr(self.0)
    }
}

impl <T> Copy for SendMutPtr<T> {

}

impl<T> Pointer for SendMutPtr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.0, f)
    }
}

impl<T> SendMutPtr<T> {
    ///
    /// Makes `ptr` Send
    ///
    /// # Safety
    /// The `ptr` parameter must be able to handle being sent to other threads
    /// or special care must be taken when using the wrapped `ptr` to not use it
    /// in any way in other threads.
    ///
    #[inline(always)]
    pub const unsafe fn new(ptr: *mut T) -> Self {
        Self(ptr)
    }
    ///
    /// Makes a Send null ptr.
    ///
    #[inline(always)]
    pub const fn null() -> Self {
        Self(std::ptr::null_mut())
    }

    ///
    /// Casts `ptr` to another data type while keeping it Send.
    ///
    #[inline(always)]
    pub const fn cast<Y>(&self) -> SendMutPtr<Y> {
        SendMutPtr(self.0.cast())
    }

    ///
    /// Returns inner `ptr` which is then no longer Send.
    ///
    #[inline(always)]
    pub const fn inner(&self) -> *mut T {
        self.0
    }

    ///
    /// Makes this `ptr` Sync
    ///
    /// # Safety
    /// This `ptr` must be able to handle being accessed by multiple threads at the same time,
    /// or special care must be taken when using the wrapped `ptr` to not use it
    /// in any way in other threads.
    ///
    #[inline(always)]
    pub const unsafe fn as_sync_const(&self) -> SyncConstPtr<T> {
        SyncConstPtr(self.0)
    }

    ///
    /// Makes this `ptr` const.
    ///
    #[inline(always)]
    pub const fn as_send_const(&self) -> SendConstPtr<T> {
        SendConstPtr(self.0)
    }

    ///
    /// Makes this `ptr` Sync
    ///
    /// # Safety
    /// This `ptr` must be able to handle being accessed by multiple threads at the same time,
    /// or special care must be taken when using the wrapped `ptr` to not use it
    /// in any way in other threads.
    ///
    #[inline(always)]
    pub const unsafe fn as_sync_mut(&self) -> SyncMutPtr<T> {
        SyncMutPtr(self.0)
    }

    ///
    /// This is equivalent to .clone() and does nothing.
    ///
    #[inline(always)]
    pub const fn as_send_mut(&self) -> SendMutPtr<T> {
        SendMutPtr(self.0)
    }
}

impl<T> Deref for SendMutPtr<T> {
    type Target = *mut T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Into<*mut T> for SendMutPtr<T> {
    #[inline(always)]
    fn into(self) -> *mut T {
        self.inner()
    }
}

impl<T> Into<*const T> for SendMutPtr<T> {
    #[inline(always)]
    fn into(self) -> *const T {
        self.inner()
    }
}

///
/// Wrapped const raw pointer that is Send but not Sync
///
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SendConstPtr<T>(*const T);

unsafe impl<T> Send for SendConstPtr<T> {}

impl<T> Clone for SendConstPtr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        SendConstPtr(self.0)
    }
}

impl <T> Copy for SendConstPtr<T> {

}

impl<T> Pointer for SendConstPtr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.0, f)
    }
}

impl<T> SendConstPtr<T> {
    ///
    /// Makes `ptr` Send
    ///
    /// # Safety
    /// The `ptr` parameter must be able to handle being sent to other threads
    /// or special care must be taken when using the wrapped `ptr` to not use it
    /// in any way in other threads.
    ///
    #[inline(always)]
    pub const unsafe fn new(ptr: *const T) -> Self {
        Self(ptr)
    }

    ///
    /// Makes a Send null ptr.
    ///
    #[inline(always)]
    pub const fn null() -> Self {
        Self(std::ptr::null())
    }

    ///
    /// Casts `ptr` to another data type while keeping it Send.
    ///
    #[inline(always)]
    pub const fn cast<Y>(&self) -> SendConstPtr<Y> {
        SendConstPtr(self.0.cast())
    }

    ///
    /// Returns inner `ptr` which is then no longer Send.
    ///
    #[inline(always)]
    pub const fn inner(&self) -> *const T {
        self.0
    }

    ///
    /// Makes this `ptr` Sync
    ///
    /// # Safety
    /// This `ptr` must be able to handle being accessed by multiple threads at the same time,
    /// or special care must be taken when using the wrapped `ptr` to not use it
    /// in any way in other threads.
    ///
    #[inline(always)]
    pub const unsafe fn as_sync_const(&self) -> SyncConstPtr<T> {
        SyncConstPtr(self.0)
    }

    ///
    /// This is equivalent to .clone() and does nothing.
    ///
    #[inline(always)]
    pub const fn as_send_const(&self) -> SendConstPtr<T> {
        SendConstPtr(self.0)
    }

    ///
    /// Makes this `ptr` Sync
    ///
    /// # Safety
    /// This `ptr` must be able to handle being accessed by multiple threads at the same time,
    /// or special care must be taken when using the wrapped `ptr` to not use it
    /// in any way in other threads.
    ///
    /// `ptr` is also marked as mutable. Writing to immutable data is usually UB.
    ///
    #[inline(always)]
    pub const unsafe fn as_sync_mut(&self) -> SyncMutPtr<T> {
        SyncMutPtr(self.0.cast_mut())
    }

    ///
    /// Makes this `ptr` mutable
    ///
    /// # Safety
    /// Writing to immutable data is UB.
    ///
    #[inline(always)]
    pub const fn as_send_mut(&self) -> SendMutPtr<T> {
        SendMutPtr(self.0.cast_mut())
    }
}

impl<T> Deref for SendConstPtr<T> {
    type Target = *const T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Into<*const T> for SendConstPtr<T> {
    #[inline(always)]
    fn into(self) -> *const T {
        self.inner()
    }
}

pub trait FromConstPtr<T>: Sized {
    ///
    /// Makes `self` immutable and Send+Sync
    ///
    /// # Safety
    /// `self` must be able to handle being sent to and used concurrently by other threads,
    /// or special care must be taken when using the wrapped `self` to not use it
    /// in any way in other threads.
    ///
    unsafe fn as_sync_const(&self) -> SyncConstPtr<T>;

    ///
    /// Makes `self` immutable and Send
    ///
    /// # Safety
    /// `self` must be able to handle being sent to other threads
    /// or special care must be taken when using the wrapped `self` to not use it
    /// in any way in other threads.
    ///
    unsafe fn as_send_const(&self) -> SendConstPtr<T>;
}

pub trait FromMutPtr<T>: FromConstPtr<T> {
    ///
    /// Makes `self` Send+Sync
    ///
    /// # Safety
    /// `self` must be able to handle being sent to and used concurrently by other threads,
    /// or special care must be taken when using the wrapped `self` to not use it
    /// in any way in other threads.
    ///
    unsafe fn as_sync_mut(&self) -> SyncMutPtr<T>;

    ///
    /// Makes `self` Send
    ///
    /// # Safety
    /// `self` must be able to handle being sent to other threads
    /// or special care must be taken when using the wrapped `self` to not use it
    /// in any way in other threads.
    ///
    unsafe fn as_send_mut(&self) -> SendMutPtr<T>;
}

impl<T> FromConstPtr<T> for *const T {
    #[inline(always)]
    unsafe fn as_sync_const(&self) -> SyncConstPtr<T> {
        SyncConstPtr(self.cast())
    }

    #[inline(always)]
    unsafe fn as_send_const(&self) -> SendConstPtr<T> {
        SendConstPtr(self.cast())
    }
}

impl<T> FromConstPtr<T> for *mut T {
    #[inline(always)]
    unsafe fn as_sync_const(&self) -> SyncConstPtr<T> {
        SyncConstPtr(self.cast())
    }

    #[inline(always)]
    unsafe fn as_send_const(&self) -> SendConstPtr<T> {
        SendConstPtr(self.cast())
    }
}

impl<T> FromMutPtr<T> for *mut T {
    #[inline(always)]
    unsafe fn as_sync_mut(&self) -> SyncMutPtr<T> {
        SyncMutPtr(self.cast())
    }

    #[inline(always)]
    unsafe fn as_send_mut(&self) -> SendMutPtr<T> {
        SendMutPtr(self.cast())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::ffi::c_void;
    use std::ptr::null_mut;

    #[test]
    fn test() {
        unsafe {
            assert_eq!(size_of::<SyncConstPtr<c_void>>(), size_of::<*mut c_void>());
            assert_eq!(align_of::<SyncConstPtr<c_void>>(), align_of::<*mut c_void>());

            let mut value = 45;
            let n: *mut u64 = &mut value;
            let x = n.as_sync_const();
            assert_eq!(x, x);
            let y = n.as_send_mut();
            let z = y.as_sync_const();
            let u = z.as_send_mut();
            assert_eq!(x, z);
            assert_eq!(u, u);

            assert_eq!(z.read(), 45);
            assert_eq!(z.inner().read(), 45);
            std::thread::spawn(move || assert!(!u.inner().is_null()))
                .join()
                .unwrap();
        }
    }

    struct RustControlStructureThatIsNowSend {
        some_handle: SendConstPtr<c_void>,
        some_rust_data: u64,
    }

    #[test]
    fn example() {
        //Some handle or pointer obtained goes here.
        let handle: *mut c_void = null_mut();
        let data: u64 = 123u64;

        let rcs = RustControlStructureThatIsNowSend {
            some_handle: unsafe { handle.as_send_const() },
            some_rust_data: data,
        };

        unsafe {
            //Every *const T and *mut T has these fn's now.
            let _sync_const: SyncConstPtr<c_void> = handle.as_sync_const(); //This is unsafe.
            let _send_const: SendConstPtr<c_void> = handle.as_send_const(); //This is unsafe.

            //Every *mut T has these fn's now.
            let _sync_mut: SyncMutPtr<c_void> = handle.as_sync_mut(); //This is unsafe.
            let _send_mut: SendMutPtr<c_void> = handle.as_send_mut(); //This is unsafe.

            //The other Ptr types have the same constructors too.
            let _send_const_null: SendMutPtr<c_void> = SendMutPtr::null(); //This is safe.
            let _send_const_new: SendMutPtr<c_void> = SendMutPtr::new(null_mut());
            //This is unsafe.
        }

        std::thread::spawn(move || {
            assert!(rcs.some_handle.is_null()); //Use boxed
            let _unwrapped: *const c_void = rcs.some_handle.inner(); //unwrap if you want
            let _unwrapped2: *const c_void = rcs.some_handle.into(); //Into<*const T> is also implemented. (*mut T too when applicable)
            let casted: SendConstPtr<usize> = rcs.some_handle.cast::<usize>(); //Cast if you want.
            unsafe {
                if !casted.is_null() {
                    //In this example this is obviously always null...
                    casted.read_volatile(); //Read if you want
                }
            }

            assert_eq!(rcs.some_rust_data, 123u64)
        })
        .join()
        .unwrap();
    }
}
