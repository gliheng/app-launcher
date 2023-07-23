use crate::{
    models,
    utils::{
        command::run_command,
        kube::deploy_service,
    },
};
use tokio::runtime::Runtime;

pub fn deploy_project(ctx: &models::Context) {
    let name = &ctx.project.name;

    info!("Deploying image: {}", name);

    let image = format!("localhost:5000/{}", name);

    run_command(&format!("docker tag {} {}", name, image), ".");
    run_command(&format!("docker push {}", image), ".");

    // Publish service to kubernete
    // run_command(&format!("kn service create {name} --image {image} --port 8080", name=name, image=image), ".");
    let rt  = Runtime::new().unwrap();
    rt.block_on(async {
        let namespace: &str = "default";
        let port: i32 = 8080;
        let url = deploy_service(name, namespace, &image, port).await;
        
    });
}

fn promote_to_production() {
    
}
