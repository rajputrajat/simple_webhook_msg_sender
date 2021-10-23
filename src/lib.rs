use reqwest::{Client, Error as ReqError, Response};
use serde::Serialize;

pub struct WebhookSender {
    url: String,
    web_client: Client,
}

impl WebhookSender {
    pub fn new<S: Into<String>>(url: S) -> Self {
        Self {
            url: url.into(),
            web_client: Client::new(),
        }
    }

    pub async fn post(&self, message: &str) -> Result<Response, ReqError> {
        let msg = get_sample_msg(message);
        self.web_client.post(&self.url).json(&msg).send().await
    }
}

fn get_sample_msg(message: &str) -> JsonMessage {
    let mut msg = get_msg_template();
    msg.attachments
        .get_mut(0)
        .unwrap()
        .content
        .body
        .get_mut(0)
        .unwrap()
        .text
        .push_str(message);
    msg
}

const fn get_msg_template() -> JsonMessage {
    JsonMessage {
        r#type: "message",
        attachments: [{
            Attachment {
                content_type: "application/vnd.microsoft.card.adaptive",
                content_url: "",
                content: {
                    Content {
                        schema: "http://adaptivecards.io/schemas/adaptive-card.json",
                        r#type: "AdaptiveCard",
                        version: "1,2",
                        body: [Body {
                            r#type: "TextBlock",
                            text: String::new(),
                        }],
                    }
                },
            }
        }],
    }
}

#[derive(Debug, Serialize)]
struct JsonMessage {
    r#type: &'static str,
    attachments: [Attachment; 1],
}

#[derive(Debug, Serialize)]
struct Attachment {
    #[serde(rename = "contentType")]
    content_type: &'static str,
    #[serde(rename = "contentUrl")]
    content_url: &'static str,
    content: Content,
}

#[derive(Debug, Serialize)]
struct Content {
    #[serde(rename = "$schema")]
    schema: &'static str,
    r#type: &'static str,
    version: &'static str,
    body: [Body; 1],
}

#[derive(Debug, Serialize)]
struct Body {
    r#type: &'static str,
    text: String,
}
