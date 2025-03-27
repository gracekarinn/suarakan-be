use diesel::prelude::*;
use diesel::PgConnection;
use crate::model::publication::*;
use crate::schema::publications;

pub struct PublicationService;

impl PublicationService {
    pub fn create_publication(
        conn: &mut PgConnection, 
        new_publication: NewPublication
    ) -> Result<Publication, diesel::result::Error> {
        diesel::insert_into(publications::table)
            .values(&new_publication)
            .get_result(conn)
    }

    pub fn get_publications(conn: &mut PgConnection) -> Result<Vec<Publication>, diesel::result::Error> {
        publications::table.load(conn)
    }

    pub fn get_publication_by_id(conn: &mut PgConnection, publication_id: i32) -> Result<Publication, diesel::result::Error> {
        publications::table.find(publication_id).first(conn)
    }

    pub fn update_publication(
        conn: &mut PgConnection, 
        publication_id: i32,
        publication: Publication
    ) -> Result<Publication, diesel::result::Error> {
        diesel::update(publications::table.find(publication_id))
            .set(&publication)
            .get_result(conn)
    }

    pub fn delete_publication(conn: &mut PgConnection, publication_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(publications::table.find(publication_id))
            .execute(conn)
    }
}