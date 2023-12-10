// @generated automatically by Diesel CLI.

diesel::table! {
    dipendente (ID_DIPEN_lng) {
        ID_DIPEN_lng -> Nullable<Integer>,
        DENOMINAZIONE_s -> Nullable<Text>,
        COGNOME_S -> Nullable<Text>,
        NOME_S -> Nullable<Text>,
        INIZIALI_DIP_S -> Nullable<Text>,
    }
}
