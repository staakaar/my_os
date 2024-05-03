use core::marker::PhantomPinned;

struct SelfReferential {
    self_ptr: *const Self,
    _pin: PhantomPinned,
}

/// Box::new()ではなく
// let mut heap_value = Box::pin(SelfReferential {
//     self_ptr: 0 as *const _,
//     _pin: PhantomPinned,
// });

// unsafe {
//     let mut_ref = Pin::as_mut(&mut heap_value);
//     Pin::get_unchecked_mut(mut_ref).self_ptr = ptr;
// }

