#![allow(non_snake_case)]
use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::dipendente)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[allow(non_snake_case)]
pub struct Dipendente {
    pub ID_DIPEN_lng: Option<i32>,
    pub DENOMINAZIONE_s: Option<String>,
    pub COGNOME_S: Option<String>,
    pub NOME_S: Option<String>,
    pub INIZIALI_DIP_S: Option<String>,
}

impl Dipendente {
    pub fn ritorna_dipendente(self) -> String {
        let my_nullo = "NULL";
        let ID_DIPEN_lng = self.ID_DIPEN_lng.unwrap_or(0);
        let DENOMINAZIONE_s = self.DENOMINAZIONE_s.unwrap_or(my_nullo.to_string());
        let COGNOME_S = self.COGNOME_S.unwrap_or(my_nullo.to_string());
        let NOME_S = self.NOME_S.unwrap_or(my_nullo.to_string());
        let INIZIALI_DIP_S = self.INIZIALI_DIP_S.unwrap_or(my_nullo.to_string());
        return format!("{} {} {} {} {}",
                ID_DIPEN_lng,
                DENOMINAZIONE_s,
                COGNOME_S,
                NOME_S,
                INIZIALI_DIP_S,
        );
    }
}