pub mod emscripten;
pub mod host;

pub use self::emscripten::{generate_emscripten_env, is_emscripten_module};
