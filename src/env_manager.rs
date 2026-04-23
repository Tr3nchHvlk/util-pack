use std::collections::HashMap;
use serde_json::Value;
use std::fs;
use std::io;
use std::ops::Index;
use std::ops::IndexMut;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct KeyMap<'a> {
    pub key_map: &'a HashMap<String, String>,
}

impl<'a> KeyMap<'a> {
    pub fn contains<IntoString>(&self, key: IntoString) -> bool where IntoString: Into<String> {
        let target_key: String = key.into();
        return self.key_map.contains_key(&target_key)
    }
}

impl<'a, IntoString> Index<IntoString> for KeyMap<'a> where IntoString: Into<String> {
    type Output=String;

    fn index(&self, index: IntoString) -> &Self::Output {
        return &self.key_map[&index.into()];
    }
}

pub struct EnvManager {
    pub key_sets: HashMap<String, Value>, 
    // <String, HashMap<String, String>>,
}

impl EnvManager {
    pub fn load_from<IntoString>(content: IntoString) -> EnvManager where IntoString: Into<String> {
        let content: String = content.into();
        let env_manager = EnvManager { key_sets: HashMap::new() };
        if let Ok(key_map) = serde_json::from_str::<HashMap<String, Value>>(&content) {
            return EnvManager {
                key_sets: key_map,
            }
        }
        return env_manager;
    }

    pub fn load_from_file<IntoString>(path: IntoString) -> EnvManager where IntoString: Into<String> {
        let path_factor: String = path.into();
        let env_manager = EnvManager { key_sets: HashMap::new() };
        if let Ok(file_content) = fs::read_to_string(path_factor) {
            if let Ok(key_map) = serde_json::from_str::<HashMap<String, Value>>(&file_content) {
                return EnvManager {
                    key_sets: key_map,
                }
            }
        }
        return env_manager;
    }

    pub fn get_ref<'a, IntoString>(&'a self, index: IntoString) -> Option<EnvReadGuard<'a>> where IntoString: Into<String> {
        let index: String = index.into();
        if self.key_sets.contains_key(&index) {
            return Option::Some(
                EnvReadGuard {
                    value: &self.key_sets[&index], 
                }
            )
        } else {
            return Option::None
        }
    }

    pub fn get_ref_deser<'a, IntoString, ValueDeser>(&'a self, index: IntoString) -> Result<EnvReadGuardDeser<'a, ValueDeser>, String> where IntoString: Into<String>, ValueDeser: DeserializeOwned {
        let index: String = index.into();
        if self.key_sets.contains_key(&index) {
            let value = &self.key_sets[&index];
            let value_deser = serde_json::from_value(value.clone()).map_err(|e| format!("{}", e))?;
            return Result::Ok(
                EnvReadGuardDeser {
                    value: value,
                    deser: value_deser, 
                }
            )
        } else {
            return Result::Err(format!("No key of this value: {} found.", &index));
        }
    }
}

pub struct EnvReadGuardDeser<'a, ValueDeser> where ValueDeser: DeserializeOwned {
    value: &'a Value, 
    deser: ValueDeser, 
}

impl<'a, ValueDeser> EnvReadGuardDeser<'a, ValueDeser> where ValueDeser: DeserializeOwned {
    pub fn take(self) -> ValueDeser {
        return self.deser
    }
}

impl<'a, ValueDeser> AsRef<ValueDeser> for EnvReadGuardDeser<'a, ValueDeser> where ValueDeser: DeserializeOwned {
    fn as_ref(&self) -> &ValueDeser {
        return &self.deser
    }
}

pub struct EnvReadGuard<'a> {
    value: &'a Value, 
}

impl<'a> EnvReadGuard<'a> {
    pub fn into_deser<ValueDeser>(self) -> EnvReadGuardDeser<'a, ValueDeser> where ValueDeser: DeserializeOwned {
        return EnvReadGuardDeser { value: self.value, deser: serde_json::from_value::<ValueDeser>(self.value.clone()).unwrap() }
    }
}