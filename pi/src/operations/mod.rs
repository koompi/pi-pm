pub mod install;
pub mod remove;
pub mod search;
pub mod update;

// Correct me!!!
// Is it a good practice to declare something in mod file?

// This is the operation viriants
// No operations should be out of varaint scope
#[derive(Debug)]
pub enum Operation {
    Install,
    Update,
    Remove,
    Search,
    Help,
}
