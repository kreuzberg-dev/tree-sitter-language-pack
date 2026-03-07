use rustler::{Error, NifResult};

#[rustler::nif]
fn available_languages() -> Vec<String> {
    ts_pack_core::available_languages()
}

#[rustler::nif]
fn has_language(name: String) -> bool {
    ts_pack_core::has_language(&name)
}

#[rustler::nif]
fn language_count() -> usize {
    ts_pack_core::language_count()
}

#[rustler::nif]
fn get_language_ptr(name: String) -> NifResult<u64> {
    let language = ts_pack_core::get_language(&name).map_err(|e| Error::Term(Box::new(format!("{e}"))))?;
    let raw_ptr = language.into_raw();
    Ok(raw_ptr as u64)
}

rustler::init!("Elixir.TreeSitterLanguagePack");
