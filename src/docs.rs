use aide::{
	openapi::{Contact, License, Server, Tag},
	transform::TransformOpenApi,
};

pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
	api.title("Lerpz API")
		.summary("An example REST application")
		.description("This API handles different operations for a blog application.")
        .tos("https://lerpz.com/tos")
		.server(Server {
			url: "https://lerpz.com/api/v1".into(),
			description: Some("Production server".into()),
            ..Default::default()
		})
        .contact(Contact {
            name: Some("Lerpz Support".into()),
            url: Some("https://support.lerpz.com".into()),
            email: Some("support@lerpz.com".into()),
            ..Default::default()
        })
        .license(License {
            name: "Apache 2.0".into(),
            identifier: Some("Apache-2.0".into()),
            url: Some("https://www.apache.org/licenses/LICENSE-2.0.txt".into()),
            ..Default::default()
        })
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
