use crate::route::{RestRequest, Resource};

pub async fn handle_rest_msg(req: RestRequest) {
    match req.resource {
        Resource::Package(p) => {
            println!("package name is {}", p.pac_name);
        },
        Resource::Scenario(s) => {
            println!("scenario name is {}", s.sce_name);
        },
    }
}