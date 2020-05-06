use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::fmt::Debug;

use super::*;

type GroupAction = lights::LightState;

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupState {
    all_on: bool,
    any_on: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group<'a, C: HTTPClient + Default> {
    #[serde(skip)]
    client: Option<&'a C>,
    #[serde(skip)]
    id: Option<u8>,
    name: String,
    lights: Vec<String>,
    #[serde(skip_serializing)]
    sensors: Vec<u8>,
    #[serde(rename = "type")]
    ty: String,
    state: GroupState,
    #[serde(skip_serializing)]
    recycle: bool,
    class: Option<String>,
    #[serde(skip_serializing)]
    action: GroupAction,
}

impl<'a, C: HTTPClient + Default + Debug> Group<'a, C> {
    pub fn get_groups(http_client: &'a C) -> Res<BTreeMap<String, Self>> {
        let resp: String = http_client.get("groups")?;
        let mut groups: BTreeMap<String, Self> = serde_json::from_str(&resp)?;
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

    pub fn delete_group(http_client: &'a C, id: u8) -> Res<()> {
        let _response = http_client.delete(&format!("groups/{}", id))?;
        Ok(())
    }

    pub fn create_group(
        http_client: &'a C,
        name: String,
        lights: Vec<u8>,
        ty: Option<String>,
        class: Option<String>,
    ) -> Res<Self> {
        // TODO: ty and class
        let body = serde_json::json!({
            "name": name,
            "lights": lights
        });

        let response = http_client.post("groups", body.to_string())?;
        let mut group: Self = serde_json::from_str(&response)?;
        // TODO: parse id
        //group.id =
        group.client = Some(http_client);
        Ok(group)
    }

    pub fn update_state(self) -> Res<Self> {
        // update group
        let state = serde_json::to_string(&self.action)?;
        self.client()
            .put(&format!("groups/{}/action", self.id()), state)?;
        // get updated group
        let resp = self.client().get(&format!("groups/{}", self.id()))?;
        let group: Self = serde_json::from_str(&resp)?;
        Ok(group)
    }

    pub fn update(self) -> Res<Self> {
        // update group
        let attributes = serde_json::to_string(&self)?;
        self.client()
            .put(&format!("groups/{}", self.id()), attributes)?;
        // get updated group
        let resp = self.client().get(&format!("groups/{}", self.id()))?;
        let group: Self = serde_json::from_str(&resp)?;
        Ok(group)
    }

    pub fn delete(self) -> Res<()> {
        self.client().delete(&format!("groups/{}", self.id()))?;
        Ok(())
    }

    pub fn id(&self) -> u8 {
        self.id.unwrap()
    }

    pub fn client(&self) -> &C {
        self.client.unwrap()
    }

    pub fn action(&mut self) -> &mut GroupAction {
        &mut self.action
    }

    pub fn ty(&self) -> &str {
        &self.ty
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests_groups {

    use super::test_common::HTTPClientMock;
    use super::*;

    #[test]
    fn get_group_ok() {
        let response = String::from(
            r#"
              {
                "name": "Kitchen",
                "lights": [
                  "1"
                ],
                "sensors": [],
                "type": "Room",
                "state": {
                  "all_on": false,
                  "any_on": false
                },
                "recycle": false,
                "class": "Kitchen",
                "action": {
                  "on": false,
                  "bri": 144,
                  "alert": "none"
                }
              } "#,
        );

        let http_client_mock = HTTPClientMock {
            body: None,
            return_string: Some(response),
            error: None,
        };
        let group = Group::get_group(&http_client_mock, 1);
        assert!(group.is_ok());
    }

    #[test]
    fn get_group_err() {
        let response = String::from("not expected response");
        let http_client_mock = HTTPClientMock {
            body: None,
            return_string: Some(response),
            error: None,
        };
        let group = Group::get_group(&http_client_mock, 1);
        assert!(group.is_err());
    }

    #[test]
    fn create_group_ok() {
        assert!(true);
    }

    #[test]
    fn create_group_err() {}
}
