use mongodb::Client;
use mongodb::Collection;
use bson::doc;
use mongodb::bson::Document;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello World");
    let client  = Client::with_uri_str("mongodb+srv://Security:security123@cluster0.bubjtss.mongodb.net/?retryWrites=true&w=majority").await?;
    let movies:  Collection<Document> = client.database("UserPass").collection::<Document>("Master");
    // let filter = doc! { "item": "xyz" };
    let filter = doc! {};
    let movie= movies.find_one(filter, None).await?;
    println!("{:?}", movie);
    println!("Connected to MongoDB");
    Ok(())
}

pub fn hello() {
    println!("Hello World");
}