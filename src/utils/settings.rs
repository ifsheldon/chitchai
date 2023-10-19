use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GPTService {
    AzureOpenAI,
    OpenAI,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpenAIModel {
    GPT35,
    GPT35_16k,
    GPT4,
    GPT4_32k,
}

impl OpenAIModel {
    pub fn all_models() -> &'static [OpenAIModel] {
        &[
            OpenAIModel::GPT35,
            OpenAIModel::GPT35_16k,
            OpenAIModel::GPT4,
            OpenAIModel::GPT4_32k,
        ]
    }

    pub fn gpt35_models() -> &'static [OpenAIModel] {
        &[
            OpenAIModel::GPT35,
            OpenAIModel::GPT35_16k,
        ]
    }

    pub fn gpt4_models() -> &'static [OpenAIModel] {
        &[
            OpenAIModel::GPT4,
            OpenAIModel::GPT4_32k,
        ]
    }
}

impl Display for OpenAIModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OpenAIModel::GPT35 => "gpt-3.5-turbo",
            OpenAIModel::GPT35_16k => "gpt-3.5-turbo-16k",
            OpenAIModel::GPT4 => "gpt-4",
            OpenAIModel::GPT4_32k => "gpt-4-32k",
        })
    }
}

impl PartialEq<str> for OpenAIModel {
    fn eq(&self, other: &str) -> bool {
        let other = other.trim().to_lowercase();
        match self {
            OpenAIModel::GPT35 => other == "gpt-3.5-turbo",
            OpenAIModel::GPT35_16k => other == "gpt-3.5-turbo-16k",
            OpenAIModel::GPT4 => other == "gpt-4",
            OpenAIModel::GPT4_32k => other == "gpt-4-32k",
        }
    }
}