pub mod interaction {
    pub trait Command {
        fn execute(&self);
    }
}