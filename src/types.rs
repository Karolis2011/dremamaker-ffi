use serde::Serialize;

#[derive(Serialize)]
pub struct TypeVar {
    pub name: String,
    pub value: dreammaker::objtree::VarValue,
    pub declaration: Option<dreammaker::objtree::VarDeclaration>
}

impl TypeVar {
    pub fn from(orig: &dreammaker::objtree::Type) -> Vec<TypeVar> {
        let mut vars = Vec::<TypeVar>::new();
        for (name, var) in &orig.vars {
            vars.push(TypeVar {
                name: name.to_string(),
                value: var.value.clone(),
                declaration: var.declaration.clone()
            });
        }
        vars
    } 
}


#[derive(Serialize)]
pub struct TypeProc {
    pub name: String,
    pub values: Vec<dreammaker::objtree::ProcValue>,
    pub declaration: Option<dreammaker::objtree::ProcDeclaration>
}



impl TypeProc {
    pub fn from(orig: &dreammaker::objtree::Type) -> Vec<TypeProc> {
        let mut procs = Vec::<TypeProc>::new();
        for (name, proc) in &orig.procs {
            procs.push(TypeProc {
                name: name.to_string(),
                values: proc.value.clone(),
                declaration: proc.declaration.clone()
            });
        }
        procs
    } 
}

// #[derive(Serialize)]
// pub enum Constant {
//     /// The literal `null`.
//     Null(Option<TreePath>),
//     /// A `new` call.
//     New {
//         /// The type to be instantiated.
//         type_: Option<Pop>,
//         /// The list of arugments to pass to the `New()` proc.
//         args: Option<Vec<(Constant, Option<Constant>)>>,
//     },
//     /// A `list` literal. Elements have optional associations.
//     List(Vec<(Constant, Option<Constant>)>),
//     /// A call to a constant type constructor.
//     Call(ConstFn, Vec<(Constant, Option<Constant>)>),
//     /// A prefab literal.
//     Prefab(Pop),
//     /// A string literal.
//     String(String),
//     /// A resource literal.
//     Resource(String),
//     /// An integer literal.
//     Int(i32),
//     /// A floating-point literal.
//     Float(f32),
// }