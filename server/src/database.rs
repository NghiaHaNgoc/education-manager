use postgrest::Postgrest;

use crate::model::{SUPABASE_URL, SUPABASE_ANON_KEY};


pub fn database_connection() -> Postgrest {
    Postgrest::new(SUPABASE_URL).insert_header("apikey", SUPABASE_ANON_KEY)

}
