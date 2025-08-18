use std::fs;
use actix_web::{ web, HttpResponse, Responder };
use crate::{
    lmdb::{ order::DBOrder, utils::DB },
    schema::order::Order,
    scripts::{
        order::{ append_to_google_sheets, update_order_in_sheets },
        utils::get_or_generate_token,
    },
};

#[derive(serde::Deserialize)]
struct ServiceAccount {
    client_email: String,
    private_key: String,
}

/// Insert a new Order
#[utoipa::path(
    post,
    path = "/orders",
    request_body = Order,
    responses(
        (status = 201, description = "Order inserted successfully"),
        (status = 500, description = "Insert error")
    )
)]
pub async fn insert_order(db: web::Data<DB>, item: web::Json<Order>) -> impl Responder {
    let order = item.into_inner();
    match db.insert(order.clone()).await {
        Ok(_) => {
            let values = Order::to_sheet1_row(&order).await;
            let file_content = fs::read_to_string("./src/service_account.json");
            let sa: ServiceAccount = serde_json::from_str(&file_content.unwrap()).unwrap();
            let access_token = get_or_generate_token(&sa.client_email, &sa.private_key).await;
            let _res = append_to_google_sheets(
                access_token.unwrap(),
                "16pzLDZosE9HIhrWRrxc8ZkWERhWf0LVnqx0SI4e_eas",
                "Sheet1!A:Z",
                values
            ).await.unwrap();
            HttpResponse::Created().finish()
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Insert error: {}", e)),
    }
}

pub async fn insert_all(db: web::Data<DB>) -> impl Responder {
    match db.insert_all().await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Insert error: {}", e)),
    }
}

/// Get single Order by id
#[utoipa::path(
    get,
    path = "/orders/{id}",
    params(("id" = String, Path, description = "Order ID")),
    responses(
        (status = 200, description = "Order found", body = Order),
        (status = 404, description = "Order not found"),
        (status = 500, description = "Get error")
    )
)]
pub async fn get_order(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    match db.get_single(path.into_inner()) {
        Ok(Some(order)) => HttpResponse::Ok().json(order),
        Ok(None) => HttpResponse::NotFound().body("Order not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Get error: {}", e)),
    }
}

/// List all Orders
#[utoipa::path(
    get,
    path = "/orders",
    responses(
        (status = 200, description = "List all orders", body = [Order]),
        (status = 500, description = "List error")
    )
)]
pub async fn list_orders(db: web::Data<DB>) -> impl Responder {
    match db.get() {
        Ok(Some(orders)) => HttpResponse::Ok().json(orders),
        Ok(None) => HttpResponse::Ok().json(Vec::<Order>::new()), // empty list instead of None
        Err(e) => HttpResponse::InternalServerError().body(format!("List error: {}", e)),
    }
}

/// Update an existing Order
#[utoipa::path(
    put,
    path = "/orders",
    request_body = Order,
    responses(
        (status = 200, description = "Order updated"),
        (status = 500, description = "Update error")
    )
)]
pub async fn update_order(db: web::Data<DB>, item: web::Json<Order>) -> impl Responder {
    let order = item.into_inner();
    println!("Updating order: {:?}", order);
    // 1. Pehle DB me Order update kar
    let _ = db.put(order.clone());
    println!("entering update_order_in_sheets");
    let values = Order::to_sheet1_row(&order).await;
    let row_number = order.row_number.unwrap_or(0);
    let file_content = fs
        ::read_to_string("./src/service_account.json")
        .expect("Failed to read service account file");
    let sa: ServiceAccount = serde_json::from_str(&file_content).unwrap();
    // println!("Service Account JSON: {}", &file_content.as_ref().unwrap().len());
    let access_token = get_or_generate_token(&sa.client_email, &sa.private_key).await;

    let values_2_d = vec![values];
    println!("Updating order in sheets with row number: {}", row_number);
    let ress = update_order_in_sheets(
        access_token.unwrap(),
        "16pzLDZosE9HIhrWRrxc8ZkWERhWf0LVnqx0SI4e_eas",
        "Sheet1!A:Z",
        row_number,
        values_2_d
    ).await;
    match ress {
        Ok(_) => println!("Order updated successfully in sheets"),
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Sheets update error: {}", e));
        }
    }

    HttpResponse::Ok().body("Order and sheets updated successfully")
}

/// Delete an Order by id
#[utoipa::path(
    delete,
    path = "/orders/{id}",
    params(("id" = String, Path, description = "Order ID")),
    responses(
        (status = 200, description = "Order deleted"),
        (status = 500, description = "Delete error")
    )
)]
pub async fn delete_order(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    match db.delete(path.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Delete error: {}", e)),
    }
}

/// Configure routes for orders
pub fn order_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::resource("/orders")
            .route(web::post().to(insert_order))
            .route(web::put().to(update_order))
            .route(web::get().to(list_orders))
    )
        .service(web::resource("/orders/insert_all").route(web::get().to(insert_all)))
        .service(
            web
                ::resource("/orders/{id}")
                .route(web::get().to(get_order))
                .route(web::delete().to(delete_order))
        );
}
