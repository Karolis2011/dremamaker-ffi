use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use dreammaker::indents::IndentProcessor;
use dreammaker::objtree::ObjectTree;
use dreammaker::objtree::TypeRef;
use dreammaker::objtree::Type;
use dreammaker::objtree::TypeVar;
use dreammaker::parser::Parser;
use dreammaker::preprocessor::Preprocessor;
use dreammaker::Context;

mod types;

#[repr(C)]
pub struct CborData {
    data: *mut u8,
    length: libc::size_t,
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_load(path_ptr: *const c_char, loaded_callback: extern "C" fn(*mut ObjectTree, *mut Context) ) {
    let path = CStr::from_ptr(path_ptr);

    let context = Context::default();
    let preprocess = Preprocessor::new(&context, std::path::Path::new(path.to_str().unwrap()).to_owned()).unwrap();
    let indents = IndentProcessor::new::<Preprocessor>(&context, preprocess);
    let mut parser = Parser::new(&context, indents);
    parser.enable_procs();
    let tree = parser.parse_object_tree();

    let boxed_tree: Box<ObjectTree> = Box::new(tree);
    let boxed_context: Box<Context> = Box::new(context);
    loaded_callback(Box::into_raw(boxed_tree), Box::into_raw(boxed_context));
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_unload(ptr: *mut ObjectTree, ptr_ctx: *mut Context) { 
    assert!(!ptr.is_null());
    assert!(!ptr_ctx.is_null());
    Box::from_raw(ptr);
    Box::from_raw(ptr_ctx);
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_objecttree_root(ptr: *mut ObjectTree) -> *mut TypeRef<'static> {
    assert!(!ptr.is_null());
    let tree: &ObjectTree = &*ptr;
    let boxed: Box<TypeRef> = Box::new(tree.root());
    Box::into_raw(boxed)
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_objecttree_itertypes(ptr: *mut ObjectTree, callback: extern fn(*mut TypeRef<'static>)) { 
    assert!(!ptr.is_null());
    let tree: &ObjectTree = &*ptr;
    for child in tree.iter_types() {
        let boxed: Box<TypeRef> = Box::new(child);
        callback(Box::into_raw(boxed));
    }
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_type_free(ptr: *mut TypeRef<'static>) { 
    assert!(!ptr.is_null());
    Box::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_type_getpath(ptr: *mut TypeRef<'static>) -> *mut libc::c_char { 
    assert!(!ptr.is_null());
    let type_obj : &Type = (&*ptr).get();
    let c_str = CString::new(type_obj.path.as_str());
    c_str.unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_type_getindex(ptr: *mut TypeRef<'static>) -> u32 { 
    assert!(!ptr.is_null());
    let index : dreammaker::objtree::NodeIndex = (&*ptr).index();
    index.index() as u32
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_type_getparentindex(ptr: *mut TypeRef<'static>) -> u32 { 
    assert!(!ptr.is_null());
    let type_obj : &Type = (&*ptr).get();
    let index : dreammaker::objtree::NodeIndex = type_obj.parent_type_index().unwrap_or(dreammaker::objtree::NodeIndex::new(u32::MAX as usize));
    index.index() as u32
}


#[no_mangle]
pub unsafe extern "C" fn dreammaker_type_isroot(ptr: *mut TypeRef<'static>) -> bool { 
    assert!(!ptr.is_null());
    let type_ref : &TypeRef = &*ptr;
    type_ref.is_root()
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_type_iterchildren(ptr: *mut TypeRef<'static>, callback: extern fn(*mut TypeRef<'static>)) { 
    assert!(!ptr.is_null());
    let type_ref : &TypeRef = &*ptr;
    for child in type_ref.children() {
        let boxed: Box<TypeRef> = Box::new(child);
        callback(Box::into_raw(boxed));
    }
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_type_varscbor(ptr: *mut TypeRef<'static>) -> CborData { 
    assert!(!ptr.is_null());
    let type_obj : &Type = (&*ptr).get();
    let mut vec : Vec<u8> = serde_cbor::to_vec(&types::TypeVar::from(type_obj)).unwrap();
    
    let result = CborData {
        data: vec.as_mut_ptr(),
        length: vec.len() as _,
    };
    std::mem::forget(vec);

    result
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_type_procscbor(ptr: *mut TypeRef<'static>) -> CborData { 
    assert!(!ptr.is_null());
    let type_obj : &Type = (&*ptr).get();
    let mut vec : Vec<u8> = serde_cbor::to_vec(&types::TypeProc::from(type_obj)).unwrap();
    
    let result = CborData {
        data: vec.as_mut_ptr(),
        length: vec.len() as _,
    };
    std::mem::forget(vec);

    result
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_cbor_free(cbor_data: CborData) { 
    assert!(!cbor_data.data.is_null());
    Box::from_raw(cbor_data.data);
}


#[no_mangle]
pub unsafe extern "C" fn dreammaker_var_free(ptr: *mut &TypeVar) { 
    assert!(!ptr.is_null());
    Box::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn str_free(ptr: *mut libc::c_char) {
    if ptr.is_null() {
        return;
    }

    CString::from_raw(ptr);
}