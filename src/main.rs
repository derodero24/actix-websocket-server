use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws::{self, CloseCode, CloseReason};

struct WsStruct;

impl Actor for WsStruct {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsStruct {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Text message {:?}", text);
                if text == "close" {
                    ctx.close(Some(CloseReason {
                        code: CloseCode::Normal,
                        description: Some("Catch close operation.".to_string()),
                    }));
                }
            }
            Ok(ws::Message::Close(_)) => {
                println!("Session has closed!!");
            }
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsStruct {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("localhost", 8080))?
        .run()
        .await
}
