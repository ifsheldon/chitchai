use serde::{Deserialize, Serialize};
use transprompt::async_openai::config::{AzureConfig, OpenAIConfig};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Auth {
    OpenAI {
        api_key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        org_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        api_base: Option<String>,
    },
    AzureOpenAI {
        api_version: String,
        deployment_id: String,
        api_base: String,
        api_key: String,
    },
}


impl Into<AzureConfig> for Auth {
    fn into(self) -> AzureConfig {
        match self {
            Auth::AzureOpenAI {
                api_version,
                deployment_id,
                api_base,
                api_key,
            } => AzureConfig::new()
                .with_api_version(api_version)
                .with_deployment_id(deployment_id)
                .with_api_base(api_base)
                .with_api_key(api_key),
            _ => panic!("Cannot convert Auth to AzureConfig, Got {:?}", self),
        }
    }
}

impl Into<OpenAIConfig> for Auth {
    fn into(self) -> OpenAIConfig {
        match self {
            Auth::OpenAI {
                api_key,
                org_id,
                api_base,
            } => {
                let config = OpenAIConfig::default().with_api_key(api_key);
                let config = if let Some(org_id) = org_id {
                    config.with_org_id(org_id)
                } else {
                    config
                };
                if let Some(api_base) = api_base {
                    config.with_api_base(api_base)
                } else {
                    config
                }
            }
            _ => panic!("Cannot convert Auth to OpenAIConfig, Got {:?}", self),
        }
    }
}
