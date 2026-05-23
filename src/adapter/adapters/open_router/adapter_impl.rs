use crate::adapter::open_router::openai_compatible::OpenAICompatibleAdapter;
use crate::resolver::Endpoint;

/// The OpenRouter API is compatible with the OpenAI API.
/// NOTE: This adapter is activated for namespaced model names (e.g., `open_router::openai/gpt-4.1`)
pub struct OpenRouterAdapter;

impl OpenAICompatibleAdapter for OpenRouterAdapter {
	const DEFAULT_API_KEY_ENV_NAME: Option<&'static str> = Some("OPEN_ROUTER_API_KEY");

	fn default_endpoint() -> Endpoint {
		const BASE_URL: &str = "https://openrouter.ai/api/v1/";
		Endpoint::from_static(BASE_URL)
	}
}
