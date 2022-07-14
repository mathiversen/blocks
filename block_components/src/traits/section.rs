use block_utils::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::{JsError, JsValue};

pub trait Section: super::Component
where
    Self: Serialize + DeserializeOwned,
{
    // The key used for
    fn get_storage_key(&self) -> String {
        let name = self.get_component_name();
        format!("section_{}", name)
    }

    // Save the current state of the section to local storage
    fn save_to_storage(&self) -> Result<(), JsValue> {
        let name = self.get_component_name();
        let data = serde_json::to_string(self).map_err(|_| {
            JsError::new(format!("Failed to create json for component: {}", &name).as_str())
        })?;
        console_log!("[ Saved ] {}: {:?}", &name, &data);
        local_storage().set_item(&self.get_storage_key(), &data)
    }

    fn get_data(&self) -> Result<Option<Self>, JsValue> {
        let name = self.get_component_name();
        let data = local_storage().get_item(&self.get_storage_key())?;

        if let Some(data) = data {
            console_log!("[ Loaded ] {}: {:?}", &name, &data);
            let parsed = serde_json::from_str::<Self>(&data).unwrap_ext();
            return Ok(Some(parsed));
        }

        Ok(None)
    }

    // Load section from storage
    fn load_from_storage(&self) -> Option<Self> {
        match self.get_data() {
            Ok(data) => data,
            Err(e) => {
                console_err!("{:?}", e);
                None
            }
        }
    }
}
