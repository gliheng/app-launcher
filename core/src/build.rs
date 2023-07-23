use crate::{
    utils::{
        path,
        command::run_workflow_commands,
    },
    models::{self, Workflow},
};

/// fetch source and build a project to image
pub fn build_project(ctx: &models::Context) -> String {
    let name = &ctx.project.name;
    let p = path::BuildInfo::new(&ctx.task_id, name).source_path();
    info!("Building project: {} at path {:?}", name, p);

    let workflow_type = &ctx.project.workflow;
    let mut workflow = Workflow { cmds: vec![] };
    if workflow_type == "static" {
        workflow.cmds = vec![String::from("pack build {name} --buildpack paketo-buildpacks/web-servers --env BP_NODE_RUN_SCRIPTS={buildScript} --env BP_WEB_SERVER=nginx --env BP_WEB_SERVER_ROOT={distDir}")];
    } else if workflow_type == "nodejs" {
        workflow.cmds = vec![String::from("pack build {name} --buildpack paketo-buildpacks/nodejs --env BP_NODE_RUN_SCRIPTS={buildScript} --env BP_NPM_START_SCRIPT={startScript} --env BP_LOG_LEVEL=DEBUG")];
    }

    run_workflow_commands(&workflow.cmds, name, ctx, p);

    return "".to_string();
}
