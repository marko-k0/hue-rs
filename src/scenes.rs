use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use serde_json;

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppData {
    version: Option<u8>,
    data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene<'a, C: HTTPClient + Default> {
    #[serde(skip)]
    client: Option<&'a C>,
    #[serde(skip)]
    id: Option<String>,
    name: String,
    #[serde(rename="type")]
    ty: String,
    group: Option<String>,
    lights: Vec<String>,
    owner: String,
    recycle: bool,
    locked: bool,
    appdata: Option<AppData>,
    picture: String,
    lastupdated: String,
    version: u8
}

/// API for operations on the scenes.
impl<'a, C: HTTPClient + Default> Scene<'a, C> {

    pub fn get_scenes(http_client: &'a C) -> Res<BTreeMap<String, Self>> {
        let resp: String = http_client.get("scenes")?;
        let mut scenes: BTreeMap<String, Self> = serde_json::from_str(&resp)?;

        for (id, scene) in scenes.iter_mut() {
            scene.id = Some(id.parse().unwrap());
            scene.client = Some(http_client);
        }

        Ok(scenes)
    }
}
