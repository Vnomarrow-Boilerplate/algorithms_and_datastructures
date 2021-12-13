// Takes a non mutable reference and makes static and mutable
pub fn fabricate_mut_ref<'t, T>(ptr: &'t T) -> &'static mut T {
    let reference = ptr as *const T;
    let ref_val = reference as usize;
    let ptr2 = ref_val as *mut u8;
    unsafe {
        return &mut *ptr2.cast();
    }
}

// Cast a pointer from one type to another
pub fn cast_ptr<FROM, TO>(array_ptr: &FROM) -> &mut TO {
    let reference = array_ptr as *const FROM;
    let ref_val = reference as usize;
    let ptr2 = ref_val as *mut u8;
    unsafe {
        return &mut *ptr2.cast();
    }
}

// Make usize into pointer
pub fn usize_to_ptr<T>(val: usize) -> &'static mut T {
    let addr = val as *mut u8;
    unsafe {
        return &mut *addr.cast();
    }
}

// Make pointer into usize
pub fn ptr_to_usize<T>(ptr: &T) -> usize {
    let reference = ptr as *const T;
    return reference as usize;
}