fn main() {
    let _subscriber = SubscriberFoundation.find(SubsriberSearchType::Id(1195));
    let _subscriber = SubscriberFoundation.find(SubsriberSearchType::Email("bss@none".to_string()));
    let _subscriber =
        SubscriberFoundation.find(SubsriberSearchType::UniqueNickName(uuid::Uuid::new_v4()));
    let _subscriber = SubscriberFoundation.find(SubsriberSearchType::Ssn("123-45-6789".to_string()));
}

struct Subscriber {}

enum SubsriberSearchType {
    Id(i32),
    Email(String),
    UniqueNickName(uuid::Uuid),
    Ssn(String),
}

struct SubscriberFoundation;

impl SubscriberFoundation {
    fn find(&self, search_type: SubsriberSearchType) -> Option<Subscriber> {
        match search_type {
            SubsriberSearchType::Id(id) => {
                println!("search by id: {}", id);
                None
            }
            SubsriberSearchType::Email(email) => {
                println!("search by email: {}", email);
                None
            }
            SubsriberSearchType::UniqueNickName(unique_nick_name) => {
                println!("search by unique nick name: {}", unique_nick_name);

                None
            }
            SubsriberSearchType::Ssn(ssn) => {
                println!("search by ssn: {}", ssn);
                None
            }
        }
    }
}
