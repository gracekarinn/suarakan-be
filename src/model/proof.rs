use diesel::prelude::*;
use crate::schema::proofs;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = proofs)]
#[diesel(primary_key(proofid))]
pub struct Proof {
    pub proofid: i32,
    pub link: String,
}

#[derive(Insertable)]
#[diesel(table_name = proofs)]
pub struct NewProof {
    pub link: String,
}