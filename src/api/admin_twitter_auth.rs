use poem::{http::StatusCode, web::Data, Error, Result};
use poem_openapi::{payload::Json, Object, OpenApi};
use serde::{Deserialize, Serialize};

use crate::{
    api::{tags::ApiTags, token::Token},
    config::Config,
    state::{DynamicConfig, DynamicConfigEntry},
    State,
};

pub struct ApiAdminTwitterAuth;

/// Github authentication config
#[derive(Debug, Object, Serialize, Deserialize, Default)]
pub struct TwitterAuthConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl DynamicConfig for TwitterAuthConfig {
    type Instance = TwitterAuthConfig;

    fn name() -> &'static str {
        "twitter-auth"
    }

    fn create_instance(self, _config: &Config) -> Self::Instance {
        TwitterAuthConfig {
            client_id: String::new(),
            client_secret: String::new(),
        }
    }
}

#[OpenApi(prefix_path = "/admin/twitter_auth", tag = "ApiTags::AdminGithubAuth")]
impl ApiAdminTwitterAuth {
    /// Set Github auth config
    #[oai(path = "/config", method = "post")]
    async fn set_config(
        &self,
        state: Data<&State>,
        token: Token,
        config: Json<TwitterAuthConfig>,
    ) -> Result<()> {
        if !token.is_admin {
            return Err(Error::from_status(StatusCode::FORBIDDEN));
        }
        state
            .set_dynamic_config::<TwitterAuthConfig>(DynamicConfigEntry {
                enabled: true,
                config: config.0,
            })
            .await?;
        Ok(())
    }

    /// Get Github auth config
    #[oai(path = "/config", method = "get")]
    async fn get_config(&self, state: Data<&State>) -> Result<Json<TwitterAuthConfig>> {
        let entry = state.load_dynamic_config::<TwitterAuthConfig>().await?;
        Ok(Json(entry.config))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_harness::TestServer;

    #[tokio::test]
    async fn set_get_twitter_oauth() {
        let server = TestServer::new().await;
        let admin_token = server.login_admin().await;

        let resp = server
            .post("/api/admin/twitter_auth/config")
            .header("X-API-Key", &admin_token)
            .body_json(&json!({
                "client_id": "twiterclient",
                "client_secret": "twitersecret",
            }))
            .send()
            .await;
        resp.assert_status_is_ok();

        let resp = server
            .get("/api/admin/twitter_auth/config")
            .header("X-API-Key", &admin_token)
            .send()
            .await;
        resp.assert_status_is_ok();

        // let body = resp.0.take_body().into_string().await.unwrap();
        // dbg!(body);

        let json = resp.json().await;
        json.value().object().get("client_id").assert_string("twiterclient");
        json.value().object().get("client_secret").assert_string("twitersecret");
    }
}
