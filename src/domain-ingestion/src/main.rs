/*
    Bir e-ticaret projesine ait alan bilgilerin (ubiquitous language,
    mimari kurallar, kod parçacıkları) LM Studio üzerinden embedding vektörlerine
    çevrilerek Qdrant vektör veritabanına yüklenmesini ele alıyoruz.

    Kod akışı tam olarak şöyle;
    JSON dosyası oku -> Her kayıt için embedding hesapla -> Qdrant db'ye ekle
*/

// anyhow: Rust'ta hata yönetimini kolaylaştıran bir kütüphane.
// Result<()> dönüş tipi ile farklı hata türlerini tek bir yerde toplayabiliriz.
use anyhow::Result;

// colored: Terminal çıktılarını renkli hale getirmek için kullanıyoruz.
use colored::Colorize;

// qdrant_client: Qdrant vektör veritabanının Rust istemcisi.
// Qdrant  -> Veritabanı bağlantı nesnesi
// Payload -> Her bir vektör noktasına eklenebilen metadata (JSON benzeri key-value)
use qdrant_client::{Qdrant, Payload};
use qdrant_client::qdrant::{
    CreateCollectionBuilder, 
    Distance,                
    PointStruct,             
    VectorParams,            
    VectorsConfig,           
    vectors_config::Config, 
};

// serde: Serileştirme işlemleri için
use serde::{Deserialize, Serialize};

use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Qdrant'taki koleksiyon adı. Tüm veri setleri bu koleksiyona yüklenir.
const COLLECTION_NAME :&str = "e_commerce_alpha_v1";
// LM Studio'nun OpenAI uyumlu embedding API adresi.
const LM_STUDIO_API_URL :&str = "http://localhost:1234/v1/embeddings";
// Kullanılan embedding modeli. Bu model metni sayısal vektörlere dönüştürür.
const TEXT_MODEL :&str ="text-embedding-nomic-embed-text-v1.5";

// Modelin ürettiği vektör boyutu. Bu değer modelden modele değişir.
// nomic-embed-text-v1.5 varsayılan olarak 768 boyutlu vektörler üretir.
const VECTOR_SIZE :usize = 768;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "=== Domain Ingestion Started ===".bold().cyan());

    // Qdrant bağlantısı tesis ediliyor.
    // gRPC protokolü üzerinden bağlanıyoruz (port 6334).
    // REST API için 6333 portu kullanılabilir (dashboard vs.)
    println!("{} Qdrant @ {}", "[CONNECT]".bold().blue(), "http://localhost:6334".yellow());
    let client = Qdrant::from_url("http://localhost:6334").build()?;
    println!("{} Qdrant ile bağlantı kuruldu", "[  OK  ]".bold().green());

    /*
        Sırada koleksiyon oluşturma işlemleri var.
        Koleksiyon zaten varsa tekrar oluşturmaya gerek yok.
        Yoksa, vektör boyutu ve mesafe metriği belirterek(ki burada kosinüs hesaplaması) 
        yeni bir koleksiyon oluşturuyoruz.
    */
    if !client.collection_exists(COLLECTION_NAME).await? {
        println!("{} '{}' koleksiyonu oluşturuluyor", "[CREATE]".bold().magenta(), COLLECTION_NAME.yellow());
        client
            .create_collection(
                CreateCollectionBuilder::new(COLLECTION_NAME)
                    .vectors_config(VectorsConfig {
                        // Config::Params tek bir vektör alanı tanımlar.
                        // Birden fazla vektör alanı için Config::ParamsMap kullanılır.
                        config: Some(Config::Params(VectorParams {
                            size: VECTOR_SIZE as u64,
                            // Cosine benzerliği: Metin embedding'leri için en yaygın mesafe ölçümü.
                            // 1.0 = tam benzer, 0.0 = ilişkisiz, -1.0 = tam zıt
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

    // Son olarak veri dosyalarını sırayla işleme alıyoruz.
    execute_file("./data/ubiquities-language.json", &client).await?;
    execute_file("./data/architecture-rules.json", &client).await?;
    execute_file("./data/semantic-code-chunks.json", &client).await?;

    println!("{}", "=== Tüm veriler işlendi ===".bold().cyan());
    Ok(())
}

/* 
    LM Studio'dan dönen yanıttaki tek bir embedding sonucunu ifade eder
    Her metin için bir embedding vektörü (f32 dizisi) döner.
*/
#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>
}

// LM Studio API'sine gönderilen istek gövdesi. OpenAI /v1/embeddings endpoint ile uyumludur.
#[derive(Serialize)]
struct EmbeddingRequest {
    input:String,   // Embedding'i hesaplanacak metin
    model:String    // Kullanılacak modelin adı
}

// LM Studio API'sinden dönen yanıt. data dizisi içinde bir veya birden fazla EmbeddingData bulunur.
#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>
}


/*
    Üç farklı JSON dosyası var. Buradaki veri yapılarını tek bir enum altında birleştiriyoruz.
    untagged flag'i sebebiyle serde, JSON nesnesinin alanlarına bakar ve
    hangi variant için deserialize işlemi yapacağına otomatik karar verir.
*/
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum DataSource {
    UbiqLang {term:String,definition:String,context:String},
    ArchRule {rule_id:String, category:String,description:String},
    CodeSemantic {file_name:String,content:String,tags:Vec<String>},
}


// LM Studio API'sini çağırarak verilen metnin embedding vektörünü hesaplar.
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

/* 
    Veri kaynağı öğesini ayrıştırarak iki şey üretir. 
    Embedding için kullanılacak birleştirilmiş metin ve Qdrant'a kaydedilecek payload (metadata)
    Payload, vektör araması sonucunda dönen noktalardaki ek bilgileri taşır.
    Böylece sadece benzer vektörleri bulmakla kalmaz, her noktanın ne olduğunu (terim mi, kural mı, kod mu) ve detaylarını da görebiliriz.
*/
fn parse_data(item: &DataSource) -> (String, Payload) {
    let mut payload = Payload::new();
    match item {
        // Ubiquitous Language: Terim + tanım birleştirilerek embedding metni oluşturulur
        DataSource::UbiqLang { term, definition, context } => {
            let text = format!("{}: {}", term, definition);
            payload.insert("type", "term");
            payload.insert("term", term.clone());
            payload.insert("definition", definition.clone());
            payload.insert("context", context.clone());
            payload.insert("content", text.clone());
            (text, payload)
        }
        // Architecture Rule: Kural ID + kategori + açıklama birleştirilir
        DataSource::ArchRule { rule_id, category, description } => {
            let text = format!("{} [{}]: {}", rule_id, category, description);
            payload.insert("type", "rule");
            payload.insert("rule_id", rule_id.clone());
            payload.insert("category", category.clone());
            payload.insert("description", description.clone());
            payload.insert("content", text.clone());
            (text, payload)
        }
        // Code Semantic: Kod içeriğinin kendisi doğrudan embedding metni olarak kullanılır
        DataSource::CodeSemantic { file_name, content, tags } => {
            payload.insert("type", "code");
            payload.insert("file_name", file_name.clone());
            payload.insert("content", content.clone());
            payload.insert("tags", tags.clone());
            (content.clone(), payload)
        }
    }
}

/*
    Kaynak dosya adı ve index'ten deterministik (tekrarlanabilir) bir u64 ID üretir
    ve bir Qdrant noktası (PointStruct) oluşturur. Deterministik olduğu için aynı veri 
    tekrar yüklendiğinde aynı ID üretilir. Dolayısıyla Qdrant'taki upsert(update veya insert)
    işlemi mevcut noktayı günceller veri tekrarı yapılmamış olur.
*/
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

// JSON dosyasını okur, her kaydı embedding'e çevirir ve Qdrant db'ye toplu olarak yükler.
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

        if i == 0 {
            println!("{} Embedding boyutu: {} (beklenen: {})",
                "[ INFO ]".bold().blue(),
                embedding.len().to_string().cyan(),
                VECTOR_SIZE.to_string().cyan());
            if embedding.len() != VECTOR_SIZE {
                eprintln!("{} Embedding boyutu uyuşmuyor! Model {} boyut döndürdü ama koleksiyon {} boyut bekliyor.",
                    "[ WARN ]".bold().red(),
                    embedding.len().to_string().red(),
                    VECTOR_SIZE.to_string().red());
            }
        }
        let point = create_qdrant_point(file_name, i, embedding, payload);
        points.push(point);
    }

    // Upsert: Insert + Update birleşimi. Aynı ID'ye sahip nokta varsa günceller, yoksa yeni ekler.
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
