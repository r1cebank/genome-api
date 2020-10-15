#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
extern crate genome;
use genome::DNA;

#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/")]
fn index() -> JsonValue {
    json!({
        "description": "DNA generation service",
        "status": "ok"
    })
}

#[derive(Deserialize)]
struct CompareData {
    dna1: String,
    dna2: String,
}

#[derive(Deserialize)]
struct MergeData {
    dna1: String,
    dna2: String,
}

#[derive(Deserialize)]
struct ZeroInputData {
    dna: String,
    position: u16,
}

#[derive(Deserialize)]
struct DecodeData {
    dna: String,
}

#[post("/compare", format = "json", data = "<data>")]
fn compare_dna(data: Json<CompareData>) -> ApiResponse {
    if !DNA::is_valid(data.dna1.clone()) || !DNA::is_valid(data.dna2.clone()) {
        return ApiResponse {
            json: json!({
                "error": "DNA string not valid"
            }),
            status: Status::BadRequest,
        };
    }
    let dna1 = DNA::from(data.dna1.clone());
    let dna2 = DNA::from(data.dna2.clone());
    ApiResponse {
        json: json!({
            "similarity": DNA::compare(dna1, dna2),
        }),
        status: Status::Ok,
    }
}

#[post("/merge", format = "json", data = "<data>")]
fn merge_dna(data: Json<MergeData>) -> ApiResponse {
    if !DNA::is_valid(data.dna1.clone()) || !DNA::is_valid(data.dna2.clone()) {
        return ApiResponse {
            json: json!({
                "error": "DNA string not valid"
            }),
            status: Status::BadRequest,
        };
    }
    let dna1 = DNA::from(data.dna1.clone());
    let dna2 = DNA::from(data.dna2.clone());
    let dna = DNA::merge(dna1, dna2, true).unwrap();
    ApiResponse {
        json: json!({
            "pool_size": dna.pool_size,
            "gene_size": dna.gene_size,
            "dna_str": dna.to_string(),
            "raw_value": dna.to_latent_vec(),
            "raw_size": dna.to_latent_vec().len()
        }),
        status: Status::Ok,
    }
}

#[post("/decode", format = "json", data = "<data>")]
fn decode_dna(data: Json<DecodeData>) -> ApiResponse {
    let dna = DNA::from(data.dna.clone());
    ApiResponse {
        json: json!({
            "pool_size": dna.pool_size,
            "gene_size": dna.gene_size,
            "dna_str": dna.to_string(),
            "raw_value": dna.to_latent_vec(),
            "raw_size": dna.to_latent_vec().len()
        }),
        status: Status::Ok,
    }
}

#[post("/zero", format = "json", data = "<data>")]
fn zero_dna(data: Json<ZeroInputData>) -> ApiResponse {
    let mut dna = DNA::from(data.dna.clone());
    if data.position >= dna.pool_size {
        return ApiResponse {
            json: json!({
                "error": "Invalid position"
            }),
            status: Status::BadRequest,
        };
    }
    dna.genes[data.position as usize].zero();
    ApiResponse {
        json: json!({
            "pool_size": dna.pool_size,
            "gene_size": dna.gene_size,
            "dna_str": dna.to_string(),
            "raw_value": dna.to_latent_vec(),
            "raw_size": dna.to_latent_vec().len()
        }),
        status: Status::Ok,
    }
}

#[get("/dna?<pool_size>&<gene_size>")]
fn get_dna(pool_size: u16, gene_size: u16) -> ApiResponse {
    if pool_size > 512 {
        return ApiResponse {
            json: json!({
                "error": "pool_size is over: 512"
            }),
            status: Status::BadRequest,
        };
    }
    if gene_size > 512 {
        return ApiResponse {
            json: json!({
                "error": "gene_size is over: 512"
            }),
            status: Status::BadRequest,
        };
    }
    let dna = DNA::new(pool_size, gene_size);
    return ApiResponse {
        json: json!({
            "pool_size": pool_size,
            "gene_size": gene_size,
            "dna_str": dna.to_string(),
            "raw_value": dna.to_latent_vec(),
            "raw_size": dna.to_latent_vec().len()
        }),
        status: Status::Ok,
    };
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, get_dna, compare_dna, decode_dna, merge_dna, zero_dna],
        )
        .launch();
}
