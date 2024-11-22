trait Logger {
    fn log(&self, message: &str);
}

struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[Console log]: {}", message);
    }
}

struct FileLogger;
impl Logger for FileLogger {
    fn log(&self, message: &str) {
        println!("[File log]: {}", message);
    }
}


struct Service<T: Logger> {
    logger: T,
}

impl<T: Logger> Service<T> {
    fn start(&self) {
        self.logger.log("Service started");
    }
}

fn main() {
    let s1 = Service { logger: ConsoleLogger };
    s1.start();

    let s2 = Service { logger: FileLogger };
    s2.start();
}