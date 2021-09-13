//! A one-file example of a microservice with a simple get/set API
//!
//! This service models a list of packages and then prints them to the main
//! landing page served by this service. There's an API for getting the JSON
//! representation of the packages as well as a POST method to add new ones.
use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

use rocket::{get, launch, routes, Rocket, Build, post, response::content};
use rocket::serde::{Serialize, Deserialize, json::Json};

/// Definition of a "package" to be printed on a page
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Package {
    name: String,
    url: String,
}

lazy_static! {
    // Global list of packages; not really good form, should have a DB
    static ref PACKAGES: Mutex<HashMap<String, Package>> = Mutex::new(HashMap::new());
}

//******************************************************************************
// Web server handlers
//******************************************************************************

/// Main server function; mounts the handlers
#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1", routes![packages_get, packages_post])
}

/// Main web landing page implementation
#[get("/")]
fn index() -> content::Html<String> {
    let mut list = vec!["<h1>Available Packages</h1>".to_string(), "<ul>".to_string()];

    let pkg_map = PACKAGES.lock().unwrap();
    for (name, pkg) in &*pkg_map {
        list.push(format!("<li><p>{}</p><p>URL: {}</p></li>", &name, &pkg.url));
    }

    list.push("</ul>".to_string());

    content::Html(list.join(""))
}

//******************************************************************************
// API handlers
//******************************************************************************

/// HTTP GET support for the package list
#[get("/packages")]
fn packages_get() -> Json<Vec<Package>> {
    let mut packages = Vec::new();

    let pkg_map = PACKAGES.lock().unwrap();
    for (_name, pkg) in pkg_map.iter() {
        packages.push(pkg.clone());
    }

    packages.into()
}

/// HTTP POST support for the package list
#[post("/packages", data="<package>")]
fn packages_post(package: Json<Package>) {
    let mut pkg_map = PACKAGES.lock().unwrap();

    let pkg = package.into_inner();

    println!("{}: {:?}", &pkg.name, &pkg);

    pkg_map.insert(pkg.name.to_string(), pkg);
}
