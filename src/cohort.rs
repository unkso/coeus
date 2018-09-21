use super::schema::cohort::{self, dsl::*};
use super::models::{Cohort, NewCohort};
use diesel::{prelude::*, result::Error};

impl Cohort {
    pub fn new_with_name(cohort_name: &str, conn: &SqliteConnection) -> Result<Self, Error> {
        let new_cohort = NewCohort { name: cohort_name };

        ::diesel::insert_into(cohort::table)
            .values(&new_cohort)
            .execute(conn)
            .and_then(|ret_id| {
                cohort.filter(id.eq(ret_id as i32)).get_result(conn)
            })
    }

    pub fn delete(&self, conn: &SqliteConnection) -> Result<usize, Error> {
        ::diesel::delete(cohort.filter(name.eq(&self.name)))
            .execute(conn)
    }
}

#[cfg(test)]
mod test {
    extern crate diesel_migrations;

    use super::Cohort;
    use diesel::{Connection, SqliteConnection};
    use self::diesel_migrations::run_pending_migrations;

    fn setup() -> SqliteConnection {
        let conn = SqliteConnection::establish(":memory:")
            .expect("Unable to establish in-memory SqliteConnection");

        run_pending_migrations(&conn).unwrap();

        conn
    }

    #[test]
    fn it_creates_a_new_cohort() {
        let conn = setup();
        let cohort = Cohort::new_with_name("test", &conn);

        assert!(cohort.is_ok());
        assert_eq!(cohort.unwrap().name, "test");
    }

    #[test]
    fn it_deletes_a_cohort() {
        let conn = setup();
        let cohort = Cohort::new_with_name("test", &conn).unwrap();

        let result = cohort.delete(&conn);
        assert!(result.is_ok());
    }
}
