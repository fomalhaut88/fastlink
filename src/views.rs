use actix_web::{get, post};
use actix_web::{web, Result, HttpResponse};
use actix_web::error::{ErrorNotFound, ErrorUriTooLong, ErrorImATeapot};
use serde::Deserialize;

use crate::appstate::AppState;
use crate::utils::{code_to_index, index_to_code};


/* Structs */

#[derive(Deserialize)]
struct AddFormData {
    url: String,
}


/* Views */

#[get("/_/version")]
async fn version() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(env!("CARGO_PKG_VERSION").to_string()))
}


#[get("/{code}")]
async fn get(web::Path(code): web::Path<String>,
             appstate: web::Data<AppState>) -> Result<HttpResponse> {
    // Return 404 if wrong length of the code
    if code.len() != appstate.order {
        return Err(ErrorNotFound(code));
    }

    // Get index from code
    let index = code_to_index(&code).unwrap();

    // Return 404 if the index in a wrong range
    if (index == 0) || (index > appstate.prime) {
        return Err(ErrorNotFound(code));
    }

    // Extract bytes of the URL from the database
    let url_buffer;
    {
        let db = appstate.db.lock().unwrap();
        url_buffer = db.get(index - 1).unwrap();
    };

    // Return 404 if the buffer is zero
    if url_buffer[0] == 0 {
        return Err(ErrorNotFound(code));
    }

    // Convert URL buffer to string
    let url = String::from_utf8(url_buffer)
        .unwrap().trim_matches('\0').to_string();

    // Return 302 with the Location=URL
    Ok(HttpResponse::Found().header("Location", url.clone()).finish())
}


#[post("/_/add")]
async fn add(form: web::Form<AddFormData>,
             appstate: web::Data<AppState>) -> Result<HttpResponse> {
    // Return 418 if URL is empty
    if form.url.is_empty() {
        return Err(ErrorImATeapot("no url"));
    }

    // Return 414 if URL is too long
    if form.url.len() > appstate.block_size {
        return Err(ErrorUriTooLong(form.url.clone()));
    }

    // Convert URL to bytes vector
    let mut url_buffer: Vec<u8> = vec![0u8; appstate.block_size];
    url_buffer[..form.url.len()].copy_from_slice(form.url.as_bytes());

    // Save URL to the database
    let state;
    {
        let mut db = appstate.db.lock().unwrap();

        // Save the bytes according to the current state
        state = db.get_state();
        db.set(state - 1, &url_buffer).unwrap();

        // Evolve and save the new state
        db.evolve_state();
        db.save_state().unwrap();
    };

    // Build code from the DB state on URL save
    let code = index_to_code(state, appstate.order);

    // Return 201 with the code in the body
    Ok(HttpResponse::Created().body(code))
}
