use kube::{
    api::{Api, Patch, PatchParams, DynamicObject},
    Client,
    core::GroupVersionKind,
    discovery::{ pinned_kind },
};
use anyhow::anyhow;
use serde_json::Value;
use std::str::FromStr;

pub async fn deploy_service(name: &str, namespace: &str, image: &str, port: i32) -> anyhow::Result<String> {
    info!("Deploy service to kubernete. name: {name}, namespace: {namespace}, image: {image}, port: {port}", name=name, namespace=namespace, image=image, port=port);
    let client = Client::try_default().await?;
    let ssapply = PatchParams::apply("kubectl-light").force();

    let yaml_str = format!(
r#"
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name:
    {name}
  namespace:
    {namespace}
spec:
  template:
    spec:
      containers:
      - image: {image}
        ports:
        - containerPort: {port}
"#, name=name, namespace=namespace, image=image, port=port);
    let obj: DynamicObject = serde_yaml::from_str(&yaml_str).unwrap();
    let gvk = GroupVersionKind::gvk("serving.knative.dev", "v1", "Service");
    let (ar, _caps) = pinned_kind(&client, &gvk).await?;
    let api: Api<DynamicObject> = Api::namespaced_with(client.clone(), namespace, &ar);
    let r = api.patch(name, &ssapply, &Patch::Apply(obj)).await?;
    let v = get_json_value(r.data, "status.url");
    v.as_str().ok_or(anyhow!("No url in kubernete response")).map(|v| v.to_string())
}

fn get_json_value(data: serde_json::Value, path: &str) -> Value {
  let p = path.split(".");
  let mut v = data;
  for name in p {
    v = match v {
      Value::Object(mut obj) => {
        if let Some(inner) = obj.remove(name) {
          inner
        } else {
          return Value::Null;
        }
      },
      Value::Array(mut arr) => {
        let i = usize::from_str(name).unwrap();
        if i < arr.len() {
          arr.remove(i)
        } else {
          return Value::Null;
        }
      },
      _ => {
        return Value::Null;
      },
    };
  }
  v
}
