#[derive(Queryable, Identifiable, Debug)]
#[table_name = "users"]
#[primary_key(id)]
pub struct QueryUser {
    pub id: u64,
    pub email: String,
    pub password: String,
    pub email: String,
    pub bio: String,
    pub image: String,
    pub token: String,
    pub created_at: u64,
    pub updated_at: u64,
}
