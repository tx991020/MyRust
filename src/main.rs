#[macro_use]
extern crate nickel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use nickel::status::StatusCode;
use nickel::{HttpRouter, JsonBody, MediaType, Nickel,FormBody};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Person {
    first_name: String,
    last_name: String,
}



fn main() {
    let mut server = Nickel::new();


    let mut t = HashMap::new();
    t.insert("a".to_owned(), "b".to_owned());
    println!("{:?}", t);

    // try it with curl
    // curl 'http://localhost:6767' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "first_name": "John","last_name": "Connor" }'
    server.post("/a", middleware! { |req, res|
        let form_body = try_with!(res, req.form_body());
        format!("Post: {:?}", form_body)
    });

    // go to http://localhost:6767/your/name to see this route in action
    server.get(
        "/:first/:last",
        middleware! { |req|
            // These unwraps are safe because they are required parts of the route
            let first_name = req.param("first").unwrap();
            let last_name = req.param("last").unwrap();

            let person = Person {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
            };
            serde_json::to_value(person).map_err(|e| (StatusCode::InternalServerError, e))
        },
    );

    // go to http://localhost:6767/raw to see this route in action
    server.get(
        "/raw",
        middleware! { |_, mut response|
            response.set(MediaType::Json);
            r#"{ "foo": "bar" }"#
        },
    );

    server.listen("127.0.0.1:6767").unwrap();
}






