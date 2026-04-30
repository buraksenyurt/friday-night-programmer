fn main() {
    let _subscriber = SubscriberFoundation.find(SubscriberSearchType::Id(1195));
    let _subscriber = SubscriberFoundation.find(SubscriberSearchType::Email("bss@none".to_string()));
    let _subscriber =
        SubscriberFoundation.find(SubscriberSearchType::UniqueNickname(uuid::Uuid::new_v4()));
    let _subscriber = SubscriberFoundation.find(SubscriberSearchType::Ssn("123-45-6789".to_string()));
}

struct Subscriber {}

enum SubscriberSearchType {
    Id(i32),
    Email(String),
    UniqueNickname(uuid::Uuid),
    Ssn(String),
}

struct SubscriberFoundation;

impl SubscriberFoundation {
    fn find(&self, search_type: SubscriberSearchType) -> Option<Subscriber> {
        match search_type {
            SubscriberSearchType::Id(id) => {
                println!("search by id: {}", id);
                None
            }
            SubscriberSearchType::Email(email) => {
                println!("search by email: {}", email);
                None
            }
            SubscriberSearchType::UniqueNickname(unique_nick_name) => {
                println!("search by unique nick name: {}", unique_nick_name);

                None
            }
            SubscriberSearchType::Ssn(ssn) => {
                println!("search by ssn: {}", ssn);
                None
            }
        }
    }
}