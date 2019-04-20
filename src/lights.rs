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
    pub fn set_on(&mut self, power: bool) -> &mut Self {
        self.on = Some(power);
        self
    }
    pub fn bri(&self) -> u8 {
        self.bri.unwrap()
    }
    pub fn set_bri(&mut self, bri: u8) -> &mut Self {
        self.bri = Some(bri);
        self
    }
    pub fn hue(&self) -> u16 {
        match self.hue {
            Some(h) => h,
            None => 0,
        }
    }
    pub fn set_hue(&mut self, hue: u16) -> &mut Self {
        if let Some(_) = self.hue {
            self.hue = Some(hue);
        }
        self
    }
    pub fn sat(&self) -> u8 {
        match self.sat {
            Some(s) => s,
            None => 0,
        }
    }
    pub fn set_sat(&mut self, sat: u8) -> &mut Self {
        if let Some(_) = self.sat {
            self.sat = Some(sat);
        }
        self
    }
    pub fn ct(&self) -> u16 {
        match self.ct {
            Some(ct) => ct,
            None => 0,
        }
    }
    pub fn set_ct(&mut self, ct: u16) -> &mut Self {
        if let Some(_) = self.ct {
            self.ct = Some(ct);
        }
        self
    }
    pub fn xy(&self) -> [f32; 2] {
        match self.xy {
            Some(xy) => xy.clone(),
            None => [0.0, 0.0],
        }
    }
    pub fn set_xy(&mut self, xy: [f32; 2]) -> &mut Self {
        if let Some(_) = self.xy {
            self.xy = Some(xy);
        }
        self
    }
    pub fn alert(&self) -> &str {
        &self.alert.as_ref().unwrap()
    }
    pub fn set_alert(&mut self, alert: &str) {
        let values = [ "none", "select", "lselect" ];
        if values.contains(&alert) {
            self.alert = Some(alert.to_owned())
        }
    }
    pub fn effect(&self) -> &str {
        &self.effect.as_ref().unwrap()
    }
    pub fn set_effect(&mut self, effect: &str) {
        let values = [ "none", "colorloop" ];
        if values.contains(&effect) {
            self.effect = Some(effect.to_owned())
        }
    }
    pub fn set_transitiontime(&mut self, time: u16) -> &mut Self {
        self.transitiontime = Some(time);
        self
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

/// API for operations on the lights.
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

    //XXX: do we need this?
    pub fn set_state(http_client: &'a Client, id: u8, state: &LightState) -> Res<String> {
        let state_json = serde_json::to_string(state)?;
        let response = http_client.put(&format!("lights/{}/state", id), state_json)?;
        Ok(response)
    }

    pub fn update(&mut self) -> Res<String> {
        //TODO: I should get a new state from API
        let state_json = serde_json::to_string(&self.state)?;
        let response = self.client().put(
            &format!("lights/{}/state", self.id()), state_json)?;
        self.state.transitiontime = None;
        Ok(response)
    }

    pub fn rename(&mut self, name: &str) -> Res<&mut Self> {
        let body = json!({"name": name});
        self.client().put(&format!("lights/{}", self.id()), body.to_string())?;
        self.name = name.to_owned();
        Ok(self)
    }

    pub fn delete(self) -> Res<()> {
        self.client().delete(&format!("lights/{}", self.id()))?;
        Ok(())
    }

    pub fn id(&self) -> u8 {
        self.id.unwrap()
    }

    pub fn client(&self) -> &Client {
        self.client.unwrap()
    }

    pub fn state(&mut self) -> &mut LightState {
        &mut self.state
    }

    pub fn ty(&self) -> &str {
        &self.ty
    }

    pub fn name(&self) -> &str {
        &self.name
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

