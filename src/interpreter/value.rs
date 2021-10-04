use core::ptr::NonNull;

use alloc::{collections::BTreeMap, string::String};

#[repr(C)]
pub struct Value {
    pub raw_value: RawValue,
    pub type_tag: TypeTag,
}

#[repr(C)]
pub union RawValue {
    pub n: (),
    pub b: bool,
    pub i: i64,
    pub u: u64,
    pub f: f64,
    pub s: NonNull<SapString>,
    pub v: NonNull<ValueVector>,
    pub o: NonNull<Object>,
    pub fun: NonNull<()>,
    pub err: NonNull<SapString>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SapString {
    pub len: usize,
    pub cap: usize,
    pub chars: NonNull<[u8]>
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ValueVector {
    pub len: usize,
    pub cap: usize,
    pub values: NonNull<[RawValue]>,
    pub types: Option<NonNull<[TypeTag]>>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Object {
    pub len: usize,
    pub cap: usize,
    pub values: NonNull<[RawValue]>,
    pub types: Option<NonNull<[TypeTag]>>,
    pub map: BTreeMap<String, usize>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub sig: (), // todo
    pub function_type_tag: FunctionTypeTag,
    pub body: NonNull<FunctionBody>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FunctionTypeTag {
    Native = 0,
    Lambda = 1,
}

#[repr(C)]
pub union FunctionBody {
    pub native: NonNull<()>,
    pub lambda: NonNull<Lambda>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Lambda {
    // todo
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TypeTag {
    Null = 0,
    Any = 1,
    Boolean,
    Int,
    Uint,
    Float,
    String,
    Array,
    Object,
    Function,
    Error,
}
