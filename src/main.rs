use rocket::*;
use business::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};
mod business;

#[get("/businesses")]
fn get_businesses()->Json<Vec<Business>>{
    let business = load_businesses();
    Json(business)
}

#[post("/business",format = "json",data="<business>")]
fn create_business(business:Json<Business>)->Status{
    let mut businesses = load_businesses();

    if let Some(_index) = businesses
       .iter()
       .position(|item| item.business_name==business.0.business_name)
       {
        return Status::Conflict;
       }
       businesses.push(business.0);
       save_businesses(&businesses);
       Status::Created
}

#[put("/business",format = "json",data="<business>")]
fn update_business(business:Json<Business>)->Status{
    let mut businesses = load_businesses();

    if let Some(index) = businesses
       .iter()
       .position(|item| item.business_name==business.0.business_name)
       {
        businesses.remove(index);
        businesses.insert(index,business.0);
        save_businesses(&businesses);
        return Status::Ok;
       }else{
        return Status::NotFound;
    }
}

#[delete("/business",format="json",data="<business>")]

fn delete_business(business:Json<Business>)->Status{
    let mut businesses = load_businesses();

    if let Some(index) = businesses
    .iter()
    .position(|item| item.business_name==business.0.business_name){
        businesses.remove(index);
        save_businesses(&businesses);
        return Status::NoContent;
    } else{
        return Status::NotFound;
    }
}
#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/business_directory.html")).await.ok()
}

#[launch]
fn rocket()-> _ {
    rocket::build().mount("/",routes![
        get_businesses,
        create_business,
        delete_business,
        update_business,
        index
    ])
}

