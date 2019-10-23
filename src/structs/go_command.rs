pub mod interaction {
    pub struct GoCommand {
        north_destination: String,
        east_destination: String,
        south_destination: String,
        west_destination: String
    }

    impl Command for GoCommand {
        fn execute(&self) {
            // TODO: handle go
        }
    }
}