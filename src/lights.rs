use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use serde_json;

use super::*;

#[derive(Builder, Serialize, Deserialize, Default, Debug)]
#[builder(default)]
#[builder(setter(into))]
pub struct LightState {
    #[serde(skip_serializing_if = "Option::is_none")]
    on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bri: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hue: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sat: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<String>, // "none", "select", "lselect"
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<String>, // "none", "colorloop"
    #[serde(skip_serializing_if = "Option::is_none")]
    xy: Option<[f32; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ct: Option<u16>,
    #[serde(skip_serializing)]
    colormode: Option<String>, // "xy", "hs", "ct"
    #[serde(skip_serializing)]
    mode: Option<String>,
    #[serde(skip_serializing)]
    reachable: Option<bool>,
    // PUT
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    transitiontime: Option<u16>, // 4 = 400 msec
}

impl LightState {
    pub fn on(&self) -> bool {
        self.on.unwrap()
    }
    pub fn set_on(&mut self, power: bool) {
        self.on = Some(power);
    }
    pub fn bri(&self) -> u8 {
        self.bri.unwrap()
    }
    pub fn set_bri(&mut self, bri: u8) {
        self.bri = Some(bri);
    }
    pub fn hue(&self) -> u16 {
        match self.hue {
            Some(h) => h,
            None => 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct LightSWUpdate {
    state: String,
    lastinstall: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Light<'a> {
    #[serde(skip)]
    client: Option<&'a Client>,
    #[serde(skip)]
    id: Option<u8>,
    state: LightState,
    swupdate: LightSWUpdate,
    #[serde(rename="type")]
    ty: String,
    name: String,
    modelid: String,
    manufacturername: String,
    productname: String,
    uniqueid: String,
    swversion: String
}

impl<'a> Light<'a> {

    pub fn get_lights(http_client: &'a Client) -> Res<BTreeMap<String,Light>> {
        let response = http_client.get("lights")?;
        let mut lights: BTreeMap<String,Light> = serde_json::from_str(&response)?;
        for (id, light) in lights.iter_mut() {
            light.id = Some(id.parse().unwrap());
            light.client = Some(http_client);
        }
        Ok(lights)
    }

    pub fn get_light(http_client: &'a Client, id: u8) -> Res<Self> {
        let response = http_client.get(&format!("lights/{}", id))?;
        let mut light: Light = serde_json::from_str(&response)?;
        light.id = Some(id);
        light.client = Some(http_client);
        Ok(light)
    }

    pub fn set_state(http_client: &'a Client, id: u8, state: &LightState) -> Res<String> {
        let state_json = serde_json::to_string(state)?;
        let response = http_client.put(&format!("lights/{}/state", id), state_json)?;
        Ok(response)
    }

    pub fn update(&self) -> Res<String> {
        let state_json = serde_json::to_string(&self.state)?;
        let response = self.client.unwrap().put(
            &format!("lights/{}/state", self.id.unwrap()), state_json)?;
        Ok(response)
    }

    pub fn rename(&mut self, name: &str) -> Res<&Self> {
        let body = json!({"name": name});
        self.client.unwrap().put(&format!("lights/{}", self.id.unwrap()), body.to_string())?;
        self.name = name.to_owned();
        Ok(self)
    }

    pub fn delete(self) -> Res<()> {
        self.client.unwrap().delete(&format!("lights/{}", self.id.unwrap()))?;
        Ok(())
    }

    pub fn test(&self) {
        let response = self.client.unwrap().get(&format!("lights/2")).unwrap();
        println!("{}", response);
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

