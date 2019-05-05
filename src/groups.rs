use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use serde_json;

use super::*;

type GroupAction = lights::LightState;

#[derive(Serialize, Deserialize, Debug)]
struct GroupState {
    all_on: bool,
    any_on: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group<'a, C: HTTPClient + Default> {
    #[serde(skip)]
    client: Option<&'a C>,
    #[serde(skip)]
    id: Option<u8>,
    lights: [u8],
    #[serde(skip_serializing)]
    sensors: [u8],
    #[serde(rename="type", default="LightGroup")]
    ty: String,
    #[serde(skip_serializing)]
    state: GroupState,
    #[serde(skip_serializing)]
    recycle: bool,
    #[serde(default="Other")]
    class: String,
    #[serde(skip_serializing)]
    action: GroupAction
}

impl<'a, C: HTTPClient + Default> Group<'a, C> {

    pub fn get_groups(http_client: &'a C) -> Res<BTreeMap<String, Self>> {
        let response = http_client.get("groups")?;
        let mut groups: BTreeMap<String,Self> = serde_json::from_str(&response)?;
        for (id, group) in groups.iter_mut() {
            group.id = Some(id.parse().unwrap());
            group.client = Some(http_client);
        }
        Ok(groups)
    }

    pub fn get_group(http_client: &'a C, id: u8) -> Res<Self> {
        let response = http_client.get(&format!("groups/{}", id))?;
        let mut group: Self = serde_json::from_str(&response)?;
        group.id = Some(id);
        group.client = Some(http_client);
        Ok(group)
    }

    pub fn update_state(self) -> Res<Self> {
        // update group
        let state_json = serde_json::to_string(&self.action)?;
        self.client().put(&format!("groups/{}/action", self.id()), state_json)?;
        // get updated group
        let response = self.client().get(&format!("groups/{}", self.id()))?;
        let group: Self = serde_json::from_str(&response)?;
        Ok(group)
    }

    pub fn update(self) -> Res<Self> {
        // update group
        let attributes_json = serde_json::to_string(&self)?;
        self.client().put(&format!("groups/{}", self.id()), attributes_json)?;
        // get updated group
        let response = self.client().get(&format!("groups/{}", self.id()))?;
        let group: Self = serde_json::from_str(&response)?;
        Ok(group)
    }

    pub fn delete(self) -> Res<()> {
        self.client().delete(&format!("groups/{}", self.id()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests_groups {

    use super::*;
    use super::test_common::HTTPClientMock;

    #[test]
    fn get_group_ok() {
        let response = String::from(r#"
        {
        "name": "Living room",
        "lights": [
            "3",
            "4"
        ],
        "sensors": [],
        "type": "Room",
        "state": {
            "all_on": false,
            "any_on": false
        },
        "recycle": false,
        "class": "Living room",
        "action": {
            "on": false,
            "bri": 144,
            "hue": 7676,
            "sat": 199,
            "effect": "none",
            "xy": [
                0.5016,
                0.4151
            ],
            "ct": 443,
            "alert": "none",
            "colormode": "xy"
        }"#);

        let http_client_mock = HTTPClientMock{
            body: None, return_string: Some(response), error: None
        };
        let group = Group::get_group(&http_client_mock, 1);
        assert!(group.is_ok());
    }

    #[test]
    fn get_group_err() {
        let response = String::from("not expected response");
        let http_client_mock = HTTPClientMock{
            body: None, return_string: Some(response), error: None
        };
        let group = Group::get_group(&http_client_mock, 1);
        assert!(group.is_err());
    }
}
