use oauth2::{EmptyExtraTokenFields, StandardTokenResponse};
use oauth2::basic::BasicTokenType;

pub trait Oauth2Session
{
	fn fromTokenResult(result: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>) -> Self;
}
