pub trait HasRepository {
    fn repository_name(&self) -> &str;
}


// impl HasRepository for Repository {
//     fn repository_name(&self) -> &str {
//         &self.full_name
//     }
// }


// impl HasRepository for PushEvent {
//     fn repository_name(&self) -> &str {
//         // self.repository.full_name
//         unimplemented!()
//     }
// }
