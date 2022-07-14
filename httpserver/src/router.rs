use http::{httprequest,httpresponse};

use crate::handler::{WebServiceHandler, StaticPageHandler, PageNotFoundHandler, Handler};
use std::io::prelude::*;


pub struct Router;

impl Router {
    pub fn route(req:httprequest::HttpRequest,stream:&mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route:Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp:httpresponse::HttpResponse = WebServiceHandler::handler(&req);
                            let _ = resp.send_response(stream);
                        },
                        _ => {
                            let resp:httpresponse::HttpResponse = StaticPageHandler::handler(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp:httpresponse::HttpResponse = PageNotFoundHandler::handler(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}