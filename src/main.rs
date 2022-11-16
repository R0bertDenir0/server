fn main() {
    

    
    
    let server= server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {

    pub struct Server {
        addr: String,
    
    }
    
    impl Server {
        pub fn new(addr: String) -> Self{
            Self { addr }
    
        }
    
        pub fn run(self) {
    
            println!("server running on port {}", self.addr)
    
        }
    }

}

mod http {
    mod request {
        struct Request {
            path: String,
            query_string: Option <String>,
            method: super::method::Method,
        }

    }
   
    mod method {
        pub enum Method {
            POST,
            GET,
            DELETE,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH
            
        }

    }
    

}

