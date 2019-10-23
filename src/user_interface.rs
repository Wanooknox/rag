pub mod ui{
    use std::io;
    use log::error;

    pub fn read_command() -> String {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => return buffer.trim().to_string(),
            Err(_) => error!("Cannot read line")
        }
        return "".to_string();
    }

    pub fn tell(text: String) {
        println!("{:?}", text);
    }
}