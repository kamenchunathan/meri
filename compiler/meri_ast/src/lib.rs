/// Contains information about the module that must be present / known for
/// every module
#[derive(Debug, PartialEq)]
pub struct ModuleHeader {
    /// module name
    pub name: String,

    // TODO: think about how to handle the imports and exports
    /// imports of the module, Currently not implemented
    pub imports: (),
}

/// The two types of modules in the language and information specific to each
/// This has to be constructed for evaluation to take place.

#[derive(Debug, PartialEq)]
pub enum ModuleType<'a> {
    /// A module meant to be the main module or another entrypoint to an
    /// executable. It contains a definition that serves as the entrypoint /
    /// main fuction
    ExecutableModule { entrypoint: Definition<'a> },

    // TODO: think about how to handle the imports and exports
    /// An ordinary module containing a list of exports
    LibraryModule { exports: Vec<()> },
}

/// A module in the meri language.
/// This is the top level structure that contains information about the
/// environment of the code and
#[derive(Debug, PartialEq)]
pub struct Module<'a> {
    pub header: ModuleHeader,
    pub typ: ModuleType<'a>,
    pub definitions: Vec<Definition<'a>>,
}

/// A defintion of a type, type alias or a function
/// only items allowed in a module
#[derive(Debug, PartialEq)]
pub enum Definition<'a> {
    TypeDefinition,
    FunctionDefinition {
        ident: Ident<'a>,
        sig: FunctionSignature<'a>,
        body: Expression,
    },
}

#[derive(Debug, PartialEq)]
pub struct FunctionSignature<'a> {
    pub params: Vec<(Pattern<'a>, Option<TypePath<'a>>)>,
    pub return_type: TypePath<'a>,
}

#[derive(Debug, PartialEq)]
pub struct TypePath<'a> {
    pub ident: Ident<'a>,
}
/// A pattern used for matching against.
/// All arguments of a function are patterns to allow destructuring of records and
/// enums in function definitions
/// e.g.
/// ```meri
///    brighten  : ( { r, g, b }: Color, intensity: Int ) => Color = {
///        ...
///    }
/// ```
// TODO: flesh out this value
#[derive(Debug, PartialEq)]
pub enum Pattern<'a> {
    /// A simple binding of the a value to a variable name
    Binding(Ident<'a>),

    /// A variant of an enum
    DataVariant,

    /// Destructuring record fields
    Record,
}

#[derive(Debug, PartialEq)]
pub struct Ident<'a>(pub &'a str);

/// Type that may be evaluated to a simpler value
// TODO: fill out
#[derive(Debug, PartialEq)]
pub enum Expression {
    Unit,
}
