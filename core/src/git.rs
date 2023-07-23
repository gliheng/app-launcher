pub fn create_repo(token: &str) {
    // curl -L \
    // -X POST \
    // -H "Accept: application/vnd.github+json" \
    // -H "Authorization: Bearer <YOUR-TOKEN>"\
    // -H "X-GitHub-Api-Version: 2022-11-28" \
    // https://api.github.com/user/repos \
    // -d '{"name":"Hello-World","description":"This is your first repo!","homepage":"https://github.com","private":false,"is_template":true}'    
}

pub fn push_repo(token: &str, dir: &str) {
    // git push https://<GITHUB_ACCESS_TOKEN>@github.com/<GITHUB_USERNAME>/<REPOSITORY_NAME>.git
}

pub fn pull_repo(token: &str, dir: &str) {

}
