use aide::{
	openapi::Tag,
	transform::TransformOpenApi,
};

pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
	api.title("Lerpz API")
		.summary("An example REST application")
		.tag(Tag {
			name: "auth".into(),
			description: Some("Authentication".into()),
			..Default::default()
		})
		.tag(Tag {
			name: "posts".into(),
			description: Some("Post Management".into()),
			..Default::default()
		})
		.tag(Tag {
			name: "health".into(),
			description: Some("Health Status".into()),
			..Default::default()
		})
}
