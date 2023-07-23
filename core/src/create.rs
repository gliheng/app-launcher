use crate::{utils::*, models};

/// Create a project from a name, name should be namespaces to avoid confliction
pub fn create_project(ctx: &models::Context) {
    info!("Creating project {}", ctx.project.name);
    let name = &ctx.project.name;
    let build = path::BuildInfo::new(&ctx.task_id, name);
    let build_dir = build.build_path();
    command::run_workflow_commands(&ctx.project.template.cmds, name, ctx, build_dir);
}
