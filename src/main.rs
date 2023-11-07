use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use sea_orm::{Database, DbErr, ConnectionTrait, DbBackend, Statement, DatabaseConnection};

use dotenv::dotenv;




async fn connect_to_database(DATABASE_URL: &str) -> Result<DatabaseConnection, DbErr>{
    let db = Database::connect(DATABASE_URL).await;
    db
}

async fn get_products(db: &DatabaseConnection, table: &str) -> Result<u64, DbErr> {
    
    let query = format!("SELECT * FROM {}", table);
    let result = db.execute(Statement::from_string(db.get_database_backend(), &query)).await?;
    let rows = result.rows_affected();
    Ok(rows)
}

async fn insert_product(db: &DatabaseConnection, product_name: &str, price: f64) -> Result<(), DbErr> {

    let query = format!("INSERT INTO demoprods (product_name, price) VALUES ('{}', {});", product_name, price);
    db.execute(Statement::from_string(db.get_database_backend(), &query)).await?;    
    Ok(())
}

async fn delete_product(db: &DatabaseConnection, product_name: &str) -> Result<(), DbErr> {

    let query = format!("DELETE FROM demoprods WHERE product_name = '{}';", product_name);
    db.execute(Statement::from_string(db.get_database_backend(), &query)).await?;

    Ok(())
}

async fn update_price(db: &DatabaseConnection, product_id: i32, new_price: f64) -> Result<(), DbErr> {

    let query = format!("UPDATE demoprods SET price = {} WHERE product_id = {};", new_price, product_id);
    db.execute(Statement::from_string(db.get_database_backend(), &query)).await?;
    Ok(())
}

async fn runDb(database_url: &str) -> Result<(), DbErr> {

    let db = connect_to_database(database_url).await?;

    let _table = "demoprods";

    get_products(&db, _table).await?;

    insert_product(&db, "Product 12", 10.0).await?;

    delete_product(&db, "test").await?;

    update_price(&db, 2, 300.52).await?;
    
    Ok(())
}


// uses Postgres, Actix, Seaorm
// inserts into /get from / delete froma database

async fn perform_action(request: HttpRequest) -> impl Responder {
    let action = request.match_info().get("action").unwrap();

    println!("{}", &action);

    let DATABASE_URL = "postgres://rust2:123456@localhost/test_rust";
    let _DB_NAME = "test_rust";
    let _TABLE = "demoprods";

    let db = connect_to_database(DATABASE_URL).await.unwrap();

    match action {
        "get" => {
            let total_products = get_products(&db, _TABLE).await.unwrap();
            format!("Total products {:?}:", &total_products)
        }
        "update" => {
            let product_id = request.match_info().get("product_id").unwrap();
            let price = request.match_info().get("price").unwrap();
            let price: f64 = price.parse().unwrap();
            let product_id: i32 = product_id.parse().unwrap();
            update_price(&db, product_id, price).await.unwrap();
            let total_products = get_products(&db, _TABLE).await.unwrap();
            format!("Total products {:?}:", &total_products)
        }
        "add" => {
            let name = request.match_info().get("name").unwrap();
            let price = request.match_info().get("price").unwrap();
            let price: f64 = price.parse().unwrap();
            insert_product(&db, name, price).await.unwrap();
            let total_products = get_products(&db, _TABLE).await.unwrap();
            format!("Total products after addition {:?}:", &total_products)
        }
        "delete" => {
            let name = request.match_info().get("name").unwrap();
            delete_product(&db, name).await.unwrap();
            let total_products = get_products(&db, _TABLE).await.unwrap();
            format!("Total products remaining {:?}:", &total_products)
        }
        _ => format!("Unknown action: {}", action),
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    
    let DATABASE_URL = "postgres://rust2:123456@localhost/test_rust";
    let _DB_NAME = "test_rust";
    let _TABLE = "demoprods";

    // let db = connect_to_database(database_url).await?;

    // let _db = runDb(&DATABASE_URL).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/{action}", web::get().to(perform_action))
            .route("/{action}/id/{product_id}/price/{price}", web::get().to(perform_action))
            .route("/{action}/name/{name}", web::get().to(perform_action))
            .route("/{action}/name/{name}/price/{price}", web::get().to(perform_action))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
