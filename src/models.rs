use diesel::Queryable;

#[derive(Debug, Clone, Queryable)]
pub struct SampleUser {
    pub id: i32,
    pub tenant_id: i32,
    pub value: String,
}
