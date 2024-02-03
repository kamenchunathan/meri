/// Contains information about the module that must be present / known for
/// every module
#[derive(Debug)]
pub struct ModuleHeader {
    /// module name
    pub name: String,

    // TODO: think about how to handle the imports and exports
    /// imports of the module, Currently not implemented
    pub imports: (),
}

/// The two types of modules in the language and information specific to each
/// This has to be constructed for evaluation to take place.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Module<'a> {
    pub header: ModuleHeader,
    pub typ: ModuleType<'a>,
    pub definitions: Vec<Definition<'a>>,
}

/// A defintion of a type, type alias or a function
/// only items allowed in a module
#[derive(Debug)]
pub enum Definition<'a> {
    TypeDefinition,
    FunctionDefinition {
        ident: &'a str,
        parameters: Vec<&'a str>,
        body: Expression,
    },
}

/// Type that may be evaluated to a simpler value
#[derive(Debug)]
pub enum Expression {
    Unit,
}
