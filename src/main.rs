use dotenv::dotenv;
use llm::{backends::google::Google, embedding::EmbeddingProvider};
use map::{Direction, Exits, Map, Room};
use text_matching::get_user_input;

mod map;
mod text_matching;

fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    if v1.len() != v2.len() {
        return 0.0; // Vectors must be of the same dimension
    }

    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let magnitude1: f32 = v1.iter().map(|a| a * a).sum::<f32>().sqrt();
    let magnitude2: f32 = v2.iter().map(|a| a * a).sum::<f32>().sqrt();

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude1 * magnitude2)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let gemini_api_key = dotenv::var("GEMINI_API_KEY").unwrap();

    let client = Google::new(
        gemini_api_key,
        Some("gemini-embedding-001".into()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    // let embeds = client
    //     .embed(vec![
    //         "What is the meaning of life?".into(),
    //         "I am asking about the meaning of life.".into(),
    //     ])
    //     .await
    //     .unwrap();
    // println!(
    //     "{}",
    //     cosine_similarity(embeds.get(0).unwrap(), embeds.get(1).unwrap())
    // );

    let root_room = Room::new_random_with_entry(
        "The room is an endless maze of peeling, \
       yellowed wallpaper and damp, stained carpet, all under the harsh, unblinking glare of a \
       humming fluorescent light. There are a few scattered electrical outlets, but no other \
       resources to be found."
            .into(),
        Direction::North,
    );
    let map = Map::new(root_room);
    loop {
        let input = get_user_input();
    }
}
