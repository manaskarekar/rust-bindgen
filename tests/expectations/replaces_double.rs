/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rooted<T> {
    pub ptr: Rooted_MaybeWrapped<T>,
}
/**
     * <div rustbindgen replaces="Rooted_MaybeWrapped"></div>
     */
pub type Rooted_MaybeWrapped<T> = T;
