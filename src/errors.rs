//! Error-related definitions generated by `error_chain`

use types::TypeName;

error_chain! {
    errors {
        /// Error thrown when the compiler is provided with a cyclic graph, as the data flow graph
        /// should always be acyclic
        CyclicGraph {
            description("graph is cyclic")
            display("graph is cyclic")
        }

        /// Error thrown when a node is not receiving the expected number of values
        WrongArgumentsCount(actual: usize, expected: usize) {
            description("wrong number of arguments")
            display("got {} arguments, expected {}", actual, expected)
        }

        /// Error thrown when accessing an invalid index in a composite type (eg. 2 in a `vec2`)
        IndexOutOfBound(index: u32, len: u32) {
            description("index out of bounds")
            display("index out of bounds ({} >= {})", index, len)
        }

        /// A more generic arguments error, usually thrown when a node receives a combination of
        /// types it cannot handle
        BadArguments(args: Box<[&'static TypeName]>) {
            description("bad arguments")
            display("bad arguments: {:?}", args)
        }

        /// Temporary error type, thrown when creating a constant with a type not yet supported
        UnsupportedConstant(ty: &'static TypeName) {
            description("unsupported constant type")
            display("unsupported constant type {:?}", ty)
        }

        /// This error is used to wrap another error with metadata about it's origin node
        BuildError(node: &'static str, id: usize) {
            description("build error")
            display("compilation failed at {} node with id {}", node, id)
        }
    }
}
