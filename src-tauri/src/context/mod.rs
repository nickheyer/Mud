pub mod forms;
pub mod parser;

use duckscript::types::runtime::Context;

// Helper function to set up context with arguments
#[allow(dead_code)]
pub fn setup_context_with_args(context: &mut Context, args: Vec<String>) {
    for (index, arg) in args.iter().enumerate() {
        context
            .variables
            .insert(format!("arg{}", index), arg.clone());
    }
}
