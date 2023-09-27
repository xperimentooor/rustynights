use auth_service::{Credentials};

fn main(){

let creds: Credentials = Credentials {
    username: String::from("itsgettinghot"),
    password: String::from("scolding"),
};

auth_service::authenticate(creds);

}