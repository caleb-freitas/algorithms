use std::ptr;

pub struct Stack<T> {
    // stores the current number of elements in the stack
    size: usize,

    // stores the offset of the top element in the stack
    offset: usize,

    // stores the maximum number of elements that the stack can hold
    capacity: usize,

    // raw pointer to the memory location where the elements of the stack are stored
    data: *mut T,
}

impl<T> Stack<T> {
    // initializes the stack by setting the pointer (data) to null, capacity, offset and
    // size to zero, and allocating memory for the stack using `libc` malloc function
    pub unsafe fn new(&mut self, capacity: usize) {
        // creates a null mutable raw pointer
        self.data = ptr::null_mut();
        self.size = 0;
        self.offset = 0;
        self.capacity = 0;

        // allocate memory for the stack
        if capacity > 0 {
            self.data = libc::malloc(capacity * std::mem::size_of::<T>()) as *mut T;
            self.capacity = capacity;
        }
    }

    // increase the capacity of the stack when the stack is full. it uses the `libc`
    // malloc function to allocate new memory, copies the elements from the old
    // memory location to a new one, and frees the old memory using libc's free function
    pub unsafe fn grow(&mut self, new_capacity: usize) {
        // allocate memory for the new data with new capacity
        let new_data = libc::malloc(new_capacity * std::mem::size_of::<T>()) as *mut T;
        let old_size = self.size;

        // copies the elements from the old memory location to the new one (new_data)
        ptr::copy_nonoverlapping(self.data, new_data, old_size);

        // frees the old memory
        libc::free(self.data as *mut libc::c_void);

        self.data = new_data;
        self.capacity = new_capacity;
    }

    // insert an element at the top of the stack
    pub unsafe fn insert(&mut self, value: T) {
        // checks if the stack is full, if it is, increase the capacity of the stack
        if self.size == self.capacity {
            self.grow(self.capacity + 2);
        }

        // overwrites a memory location with the given value
        ptr::write(self.data.offset(self.offset as isize), value);

        // define new values for `size` and `offset`
        self.size += 1; 
        self.offset = (self.offset + 1) % self.capacity;
    }

    // remove the top element from the stack
    pub unsafe fn delete(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        self.offset = (self.offset + self.capacity - 1) % self.capacity;

        Some(ptr::read(self.data.offset(self.offset as isize)))
    }

    // properly deallocate the memory used by the stack
    pub unsafe fn drop(&mut self) {
        while self.size > 0 {
            self.size -= 1;
            self.offset = (self.offset + self.capacity - 1) % self.capacity;

            // executes the destructor (if any) of the pointed-to value for every
            // element in the stack
            ptr::drop_in_place(self.data.offset(self.offset as isize));
        }
        // frees the memory allocated for the stack
        libc::free(self.data as *mut libc::c_void);
    }
}
