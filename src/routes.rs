use rocket::*;

#[get("/hello/<name>")]
fn hello(name:String)->String{
    format!("Hello {}!",name)
}
#[launch]
fn rocket()-> _ {
    rocket::build().mount("/",routes![hello])
}