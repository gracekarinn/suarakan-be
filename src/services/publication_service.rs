use diesel::prelude::*;
use diesel::PgConnection;
use crate::model::publication::Publication;
use crate::schema::publications;

pub struct PublicationService;

impl PublicationService {
    pub fn delete_publication(conn: &mut PgConnection, publication_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(publications::table.find(publication_id))
            .execute(conn)
    }
    
    pub fn get_publication_by_id(conn: &mut PgConnection, publication_id: i32) -> Result<Publication, diesel::result::Error> {
        publications::table.find(publication_id).first(conn)
    }
}