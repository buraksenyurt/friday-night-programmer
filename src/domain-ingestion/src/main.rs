use anyhow::Result;
use colored::Colorize;
use qdrant_client::{Qdrant, Payload};
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct, VectorParams, VectorsConfig,
    vectors_config::Config,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const COLLECTION_NAME :&str = "e_commerce_alpha_v1"; // Veritabanındaki koleksiyonun adı
const LM_STUDIO_API_URL :&str = "http://localhost:1234/v1/embeddings"; // LM Studio API'sinin URL'si
const TEXT_MODEL :&str ="text-embedding-nomic-embed-text-v1.5"; // Embedding modeli adı
const VECTOR_SIZE :usize = 2048; // Bu bilgiyi modelden öğrenmek lazım. 768, 1024 gibi değerler de olabilir

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "=== Domain Ingestion Started ===".bold().cyan());

    println!("{} Qdrant @ {}", "[CONNECT]".bold().blue(), "http://localhost:6334".yellow());
    let client = Qdrant::from_url("http://localhost:6334").build()?;
    println!("{} Qdrant ile bağlantı kuruldu", "[  OK  ]".bold().green());

    if !client.collection_exists(COLLECTION_NAME).await? {
        println!("{} '{}' koleksiyonu oluşturuluyor", "[CREATE]".bold().magenta(), COLLECTION_NAME.yellow());
        client
            .create_collection(
                CreateCollectionBuilder::new(COLLECTION_NAME)
                    .vectors_config(VectorsConfig {
                        config: Some(Config::Params(VectorParams {
                            size: VECTOR_SIZE as u64,
                            distance: Distance::Cosine.into(),
                            ..Default::default()
                        })),
                    })
                    .build(),
            )
            .await?;
        println!("{} '{}' koleksiyonu oluşturuldu (vector_size={}, distance=Cosine)",
            "[  OK  ]".bold().green(), COLLECTION_NAME.yellow(), VECTOR_SIZE.to_string().cyan());
    } else {
        println!("{} '{}' koleksiyonu zaten mevcut, oluşturma atlanıyor",
            "[ SKIP ]".bold().yellow(), COLLECTION_NAME.yellow());
    }

    execute_file("data/ubiq_lang.json", &client).await?;
    execute_file("data/arch_rules.json", &client).await?;
    execute_file("data/code_semantic.json", &client).await?;

    println!("{}", "=== Tüm veriler işlendi ===".bold().cyan());
    Ok(())
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>
}

#[derive(Serialize)]
struct EmbeddingRequest {
    input:String,
    model:String
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>
}

// Tüm verisetini aşağıdaki enum ile tarifleyebiliriz
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum DataSource {
    UbiqLang {term:String,definition:String,context:String},
    ArchRule {rule_id:String, category:String,description:String},
    CodeSemantic {file_name:String,content:String,tags:Vec<String>},
}

// Verisetindeki her bir kaydı embedding değerlerini bulmak için bu fonksiyon kullanılacak
async fn get_embedding(client:&reqwest::Client, input: &str) -> Result<Vec<f32>> {
    let request_body = EmbeddingRequest {
        input: input.to_string(),
        model: TEXT_MODEL.to_string(),
    };

    let response = client.post(LM_STUDIO_API_URL)
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let embedding_response: EmbeddingResponse = response.json().await?;
        Ok(embedding_response.data.into_iter().next().unwrap().embedding)
    } else {
        let status = response.status();
        eprintln!("{} Embedding API call failed with status: {}",
            "[ FAIL ]".bold().red(), status.to_string().red());
        Err(anyhow::anyhow!("Embedding API çağrısı başarısız oldu: {}", status))
    }
}

fn parse_data(item: &DataSource) -> (String, Payload) {
    let mut payload = Payload::new();
    match item {
        DataSource::UbiqLang { term, definition, context } => {
            let text = format!("{}: {}", term, definition);
            payload.insert("type", "term");
            payload.insert("term", term.clone());
            payload.insert("definition", definition.clone());
            payload.insert("context", context.clone());
            payload.insert("content", text.clone());
            (text, payload)
        }
        DataSource::ArchRule { rule_id, category, description } => {
            let text = format!("{} [{}]: {}", rule_id, category, description);
            payload.insert("type", "rule");
            payload.insert("rule_id", rule_id.clone());
            payload.insert("category", category.clone());
            payload.insert("description", description.clone());
            payload.insert("content", text.clone());
            (text, payload)
        }
        DataSource::CodeSemantic { file_name, content, tags } => {
            payload.insert("type", "code");
            payload.insert("file_name", file_name.clone());
            payload.insert("content", content.clone());
            payload.insert("tags", tags.clone());
            (content.clone(), payload)
        }
    }
}

fn create_qdrant_point(
    source: &str,
    index: usize,
    embedding: Vec<f32>,
    payload: Payload,
) -> PointStruct {
    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    index.hash(&mut hasher);
    let id = hasher.finish();

    PointStruct::new(id, embedding, payload)
}

async fn execute_file(file_name: &str, client: &Qdrant) -> Result<()> {
    println!("\n{} '{}' dosyası işleniyor", "[ FILE ]".bold().blue(), file_name.yellow());

    let file_content = fs::read_to_string(file_name)?;
    let data_items: Vec<DataSource> = serde_json::from_str(&file_content)?;
    println!("{} {} öğe '{}' dosyasından yüklendi.'",
        "[ READ ]".bold().blue(), data_items.len().to_string().cyan(), file_name.yellow());

    let http_client = reqwest::Client::new();
    let mut points = Vec::new();
    let total = data_items.len();

    for (i, item) in data_items.iter().enumerate() {
        let (text_to_embed, payload) = parse_data(item);
        let preview: String = text_to_embed.chars().take(50).collect();
        println!("{} [{}/{}] embedding işlemi yapılıyor: {}...",
            "[EMBED ]".bold().magenta(), (i + 1).to_string().cyan(), total.to_string().cyan(), preview.dimmed());
        let embedding = get_embedding(&http_client, &text_to_embed).await?;
        let point = create_qdrant_point(file_name, i, embedding, payload);
        points.push(point);
    }

    println!("{} {} points '{}' koleksiyonuna ekleniyor",
        "[UPSERT]".bold().blue(), total.to_string().cyan(), COLLECTION_NAME.yellow());
    client
        .upsert_points(
            qdrant_client::qdrant::UpsertPointsBuilder::new(
                COLLECTION_NAME.to_string(),
                points,
            )
            .build(),
        )
        .await?;
    println!("{} '{}' işlendi ve yüklendi ({} points)",
        "[  OK  ]".bold().green(), file_name.yellow(), total.to_string().cyan());
    
    Ok(())
}
