use postgrest::Postgrest;

use crate::model::{SUPABASE_ANON_KEY, SUPABASE_URL};

pub fn database_connection() -> Postgrest {
    Postgrest::new(SUPABASE_URL).insert_header("apikey", SUPABASE_ANON_KEY)
}
