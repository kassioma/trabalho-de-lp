// src/services/notes.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Array, Object, Reflect, Function};
use crate::models::note::Note;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = db)]
    pub static DB: JsValue;
}

pub struct NotesService;

impl NotesService {
    pub async fn create_note(note: &Note) -> Result<String, String> {
        let notes_ref = Self::get_collection("notes")?;
        let note_obj = Self::note_to_js(note)?;
        
        let add_fn = Reflect::get(&notes_ref, &JsValue::from_str("add"))
            .map_err(|_| "Método add não encontrado")?;
        let add_fn: Function = add_fn.into();
        
        let promise = add_fn.call1(&notes_ref, &note_obj)
            .map_err(|e| format!("Erro ao chamar add: {:?}", e))?;
        
        let result = JsFuture::from(js_sys::Promise::from(promise)).await
            .map_err(|e| format!("Erro ao criar nota: {:?}", e))?;
        
        let id = Reflect::get(&result, &JsValue::from_str("id"))
            .map_err(|_| "ID não encontrado")?
            .as_string()
            .ok_or("ID inválido")?;
        
        Ok(id)
    }
    
    pub async fn get_user_notes(user_id: &str) -> Result<Vec<Note>, String> {
        let notes_ref = Self::get_collection("notes")?;
        let user_id_val = JsValue::from_str(user_id);
        let query = Self::where_clause(&notes_ref, "userId", "==", &user_id_val)?;
        
        let get_fn = Reflect::get(&query, &JsValue::from_str("get"))
            .map_err(|_| "Método get não encontrado")?;
        let get_fn: Function = get_fn.into();
        
        let promise = get_fn.call0(&query)
            .map_err(|e| format!("Erro ao chamar get: {:?}", e))?;
        
        let snapshot = JsFuture::from(js_sys::Promise::from(promise)).await
            .map_err(|e| format!("Erro ao buscar notas: {:?}", e))?;
        
        let docs = Reflect::get(&snapshot, &JsValue::from_str("docs"))
            .map_err(|_| "Docs não encontrado")?;
        
        let docs_array: Array = docs.into();
        let mut notes = Vec::new();
        
        for doc in docs_array.iter() {
            if let Ok(note) = Self::js_to_note(&doc) {
                notes.push(note);
            }
        }
        
        Ok(notes)
    }
    
    pub async fn update_note(note: &Note) -> Result<(), String> {
        let note_id = note.id.as_ref().ok_or("ID da nota não encontrado")?;
        let notes_ref = Self::get_collection("notes")?;
        let doc_ref = Self::get_doc(&notes_ref, note_id)?;
        let note_obj = Self::note_to_js(note)?;
        
        let set_fn = Reflect::get(&doc_ref, &JsValue::from_str("set"))
            .map_err(|_| "Método set não encontrado")?;
        let set_fn: Function = set_fn.into();
        
        let promise = set_fn.call1(&doc_ref, &note_obj)
            .map_err(|e| format!("Erro ao chamar set: {:?}", e))?;
        
        JsFuture::from(js_sys::Promise::from(promise)).await
            .map_err(|e| format!("Erro ao atualizar nota: {:?}", e))?;
        
        Ok(())
    }
    
    pub async fn delete_note(note_id: &str) -> Result<(), String> {
        let notes_ref = Self::get_collection("notes")?;
        let doc_ref = Self::get_doc(&notes_ref, note_id)?;
        
        let delete_fn = Reflect::get(&doc_ref, &JsValue::from_str("delete"))
            .map_err(|_| "Método delete não encontrado")?;
        let delete_fn: Function = delete_fn.into();
        
        let promise = delete_fn.call0(&doc_ref)
            .map_err(|e| format!("Erro ao chamar delete: {:?}", e))?;
        
        JsFuture::from(js_sys::Promise::from(promise)).await
            .map_err(|e| format!("Erro ao deletar nota: {:?}", e))?;
        
        Ok(())
    }
    
    fn get_collection(name: &str) -> Result<JsValue, String> {
        let collection_fn = Reflect::get(&DB, &JsValue::from_str("collection"))
            .map_err(|_| "Método collection não encontrado")?;
        let collection_fn: Function = collection_fn.into();
        
        collection_fn.call1(&DB, &JsValue::from_str(name))
            .map_err(|e| format!("Erro ao chamar collection: {:?}", e))
    }
    
    fn get_doc(collection: &JsValue, id: &str) -> Result<JsValue, String> {
        let doc_fn = Reflect::get(collection, &JsValue::from_str("doc"))
            .map_err(|_| "Método doc não encontrado")?;
        let doc_fn: Function = doc_fn.into();
        
        doc_fn.call1(collection, &JsValue::from_str(id))
            .map_err(|e| format!("Erro ao chamar doc: {:?}", e))
    }
    
    fn where_clause(collection: &JsValue, field: &str, op: &str, value: &JsValue) -> Result<JsValue, String> {
        let where_fn = Reflect::get(collection, &JsValue::from_str("where"))
            .map_err(|_| "Método where não encontrado")?;
        let where_fn: Function = where_fn.into();
        
        where_fn.call3(collection, &JsValue::from_str(field), &JsValue::from_str(op), value)
            .map_err(|e| format!("Erro ao chamar where: {:?}", e))
    }
    
    fn note_to_js(note: &Note) -> Result<JsValue, String> {
        let obj = Object::new();
        
        Reflect::set(&obj, &"title".into(), &JsValue::from_str(&note.title))
            .map_err(|_| "Erro ao definir título")?;
        Reflect::set(&obj, &"content".into(), &JsValue::from_str(&note.content))
            .map_err(|_| "Erro ao definir conteúdo")?;
        Reflect::set(&obj, &"userId".into(), &JsValue::from_str(&note.user_id))
            .map_err(|_| "Erro ao definir userId")?;
        Reflect::set(&obj, &"createdAt".into(), &JsValue::from_f64(note.created_at as f64))
            .map_err(|_| "Erro ao definir createdAt")?;
        Reflect::set(&obj, &"updatedAt".into(), &JsValue::from_f64(note.updated_at as f64))
            .map_err(|_| "Erro ao definir updatedAt")?;
        
        Ok(obj.into())
    }
    
    fn js_to_note(doc: &JsValue) -> Result<Note, String> {
        let id = Reflect::get(doc, &"id".into())
            .ok()
            .and_then(|v| v.as_string());
        
        let data = Reflect::get(doc, &"data".into())
            .map_err(|_| "Data não encontrado")?;
        
        let data_fn: js_sys::Function = data.into();
        let data_obj = data_fn.call0(doc)
            .map_err(|_| "Erro ao chamar data()")?;
        
        let title = Reflect::get(&data_obj, &"title".into())
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
        
        let content = Reflect::get(&data_obj, &"content".into())
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
        
        let user_id = Reflect::get(&data_obj, &"userId".into())
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
        
        let created_at = Reflect::get(&data_obj, &"createdAt".into())
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as i64;
        
        let updated_at = Reflect::get(&data_obj, &"updatedAt".into())
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as i64;
        
        Ok(Note {
            id,
            title,
            content,
            user_id,
            created_at,
            updated_at,
        })
    }
}
