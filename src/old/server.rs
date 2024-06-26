// use crate::config::Config;

// use crate::core::http::http_request::HttpRequest;

// use std::collections::HashMap;
// use std::net::{TcpListener, TcpStream};

// pub struct Server {
//     config: Config,
//     connection: Option<TcpListener>,
//     routes: HashMap<String, Box<dyn Fn(&mut Context) + 'static>>,
// }

// impl Server {
//     /**
//         Create a new server instance.
//     */
//     pub fn new(host: &str, port: u16) -> Self {
//         let config = Config::new(host, port);
//         let domain = config.address();
//         let connection = TcpListener::bind(&domain).ok();
//         let routes = HashMap::new();
//         Server {
//             connection,
//             config,
//             routes,
//         }
//     }

//     /**
//        Start the server and listen for incoming connections.
//        This method will block the current thread until the server.
//     */
//     pub fn start(&self) {
//         println!("\n[ + - + - + - + - + - + - + - + - + - + - + - + - + ]\n");
//         println!("[server] started!");
//         self.config.print();
//         match &self.connection {
//             Some(listener) => self.accept(&listener),
//             None => eprintln!("[server] error: unable to bind to address"),
//         }
//     }

//     /**
//         Register a route handler.
//     */
//     pub fn route<F>(&mut self, path: &str, handler: F)
//     where
//         F: Fn(&mut Context) + 'static,
//     {
//         self.routes.insert(path.to_string(), Box::new(handler));
//     }

//     fn accept(&self, listener: &TcpListener) {
//         for stream in listener.incoming() {
//             match stream {
//                 Ok(stream) => self.handle_stream(&stream),
//                 Err(error) => eprintln!("[server] accept error: {}", error),
//             }
//         }
//     }

//     fn handle_stream(&self, tcp_stream: &TcpStream) {
//         let mut request = HttpRequest::from(&tcp_stream);
//         request.info();
//     }

//     // fn handle_stream(&self, stream: &TcpStream) {
//     //     let request = Request::init(stream);
//     //     // req.info();
//     //     let url = req.url().unwrap_or("index.html");
//     //     println!("[server] handle request: {}", url);
//     //     let cnf = Config::new(self.config.host.as_str(), self.config.port);
//     //     let mut ctx = Context::new(cnf, &req, res);
//     //     println!("[server] locating file: {:?}", &url);
//     //     println!("[server] routes: {:?}", self.routes.keys());
//     //     println!("[  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  ]");
//     //     let path = url;
//     //     let file_path = format!("./src/public/{}", path.trim_matches('/')).to_string();
//     //     println!("file path: {}", file_path);

//     //     match self.routes.get(path) {
//     //         Some(handler) => handler(ctx.borrow_mut()),
//     //         None => {
//     //             eprintln!("[server] route not found: {}", path);
//     //             let sent_asset = ctx.response.file(path);
//     //             if sent_asset.is_ok() {
//     //                 ctx.response.status(200);
//     //                 let _ = ctx.response.send();
//     //             } else if sent_asset.is_err() {
//     //                 eprintln!("[server] file not found: {:?}", sent_asset.err());
//     //                 ctx.response.content_type(ContentType::HTML);
//     //                 let _ = ctx.response.file("./src/public/404.html");
//     //                 ctx.response.status(404);
//     //                 let _ = ctx.response.send();
//     //             }
//     //         }
//     //     }
//     // }

//     //     match self.routes.get(path) {
//     //         Some(handler) => handler(ctx.borrow_mut()),
//     //         None => match std::fs::read_to_string(file_path) {
//     //             Ok(content) => {
//     //                 println!("[server] serving file: {}", path);
//     //                 if path.ends_with(".css") {
//     //                     ctx.response.content_type(ContentType::CSS);
//     //                 } else if path.ends_with(".png") {
//     //                     ctx.response.content_type(ContentType::PNG);
//     //                 } else if path.ends_with(".json") {
//     //                     ctx.response.content_type(ContentType::JSON);
//     //                 } else {
//     //                     ctx.response.content_type(ContentType::HTML);
//     //                 }
//     //                 ctx.response.body(content);
//     //                 ctx.response.status(200);
//     //                 let _ = ctx.response.send();
//     //             }
//     //             Err(err) => {
//     //                 println!("[server] file not found: {:?}", err);
//     //                 ctx.response.content_type(ContentType::HTML);
//     //                 ctx.response.file("./src/public/404.html");
//     //                 ctx.response.status(404);
//     //                 let _ = ctx.response.send();
//     //             }
//     //         },
//     //     }
//     // }

//     // fn catch_all(&self, stream: &TcpStream) {
//     //     // let non_blocking = stream.set_nonblocking(true);
//     //     // if non_blocking.is_err() {
//     //     //     eprintln!("[server] error: unable to set non-blocking mode");
//     //     //     return;
//     //     // }

//     //     println!("[server] stream: {:?}", stream);
//     //     let request = Request::init(stream);
//     //     request.info();

//     //     let mut response = Response::new(stream.try_clone().unwrap());
//     //     response.content_type(ContentType::HTML);
//     //     response.header("X-Server-Version", "0.0.1");
//     //     response.body("Hello, world!".to_string());
//     //     response.status(200);
//     //     let output = response.send();
//     //     println!("[server] response: {:?}", output);
//     // }
// }
