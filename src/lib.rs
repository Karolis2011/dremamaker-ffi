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

#[no_mangle]
pub unsafe extern "C" fn dreammaker_load(path_ptr: *const c_char) -> *mut ObjectTree {
    let path = CStr::from_ptr(path_ptr);

    let context = Context::default();
    let preprocess = Preprocessor::new(&context, std::path::Path::new(path.to_str().unwrap()).to_owned()).unwrap();
    let indents = IndentProcessor::new::<Preprocessor>(&context, preprocess);
    let mut parser = Parser::new(&context, indents);
    parser.enable_procs();
    let tree = parser.parse_object_tree();

    let boxed: Box<ObjectTree> = Box::new(tree);
    Box::into_raw(boxed)
}

#[no_mangle]
pub unsafe extern "C" fn dreammaker_unload(ptr: *mut ObjectTree) { 
    assert!(!ptr.is_null());
    Box::from_raw(ptr);
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
pub unsafe extern "C" fn dreammaker_type_itervars(ptr: *mut TypeRef<'static>, callback: extern fn(*mut libc::c_char, *mut &TypeVar)) { 
    assert!(!ptr.is_null());
    let type_obj : &Type = (&*ptr).get();
    for (name, var) in &type_obj.vars {
        let boxed: Box<&TypeVar> = Box::new(var);
        let c_str = CString::new((*name).as_str());
        let c_str_ptr = c_str.unwrap().into_raw();
        callback(c_str_ptr, Box::into_raw(boxed));
        str_free(c_str_ptr);
    }
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