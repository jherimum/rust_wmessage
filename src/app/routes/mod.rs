pub mod apikey;
pub mod plugins;
pub mod registrations;

/*
pub fn app_config(config: &mut ServiceConfig) {
    let registration = web::resource("/api/registrations").route(web::post().to(register));
    let create_channel = web::resource("/api/workspaces/{workspace_id}/channels")
        .route(web::post().to(create_channel));
    config.service(registration).service(create_channel);
}
*/
