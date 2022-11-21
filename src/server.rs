use std::net::TcpListener;
use std::io::Read;

pub struct Server {
        addr: String,
    
    }
    
    impl Server {
        pub fn new(addr: String) -> Self{
            Self { addr }
    
        }
    
        pub fn run(self) {
    
            println!("server running on port {}", self.addr);
            
            let listener = TcpListener::bind(&self.addr).unwrap();

            loop {
                match listener.accept(){
                    Ok((mut stream, _)) => {
                        let mut buffer = [0; 1024];
                       
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                println!("Received a request : {}", String::from_utf8_lossy(&buffer));
                            },
                            Err(e) => println!("failed to connect  {}", e),

                        }
                        println!("OK!")
                    }
                    Err(e) => println!("failed to established a connection : {}", e),
                }
            }

        

            
    
        }

        
    }

