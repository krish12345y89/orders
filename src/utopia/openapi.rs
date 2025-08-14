use utoipa::OpenApi;

use crate::{routes::order::*, schema::order::Order};

#[derive(OpenApi)]
#[openapi(
    paths(

        insert_order,
        get_order,
        list_orders,
        update_order,
        delete_order

    ),
    components(schemas(Order))
)]

pub struct ApiDoc;