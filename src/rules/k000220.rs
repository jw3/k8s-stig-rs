use std::process::Command;

use crate::Rule;

/// The Kubernetes Controller Manager must create unique service accounts for each work payload.
/// The Kubernetes Controller Manager is a background process that embeds core control loops regulating cluster system state through the API Server.
/// Every process executed in a pod has an associated service account. By default, service accounts use the same credentials for authentication.
/// Implementing the default settings poses a High risk to the Kubernetes Controller Manager. Setting the use- service-account-credential value lowers
/// the attack surface by generating unique service accounts settings for each controller instance.
/// CNTR-K8-000220
/// Category I
impl<'a> Rule for crate::K000220<'a> {
    type ErrType = String;

    fn id(&self) -> String {
        "CNTR-K8-000220".into()
    }


    /// Change to the /etc/kubernetes/manifests directory on the Kubernetes Master Node. Run the command: grep -i use-service-account-credential
    /// If the setting use-service-account-credential is not set in the Kubernetes Controller Manager manifest file or it is set to “false”, this is a finding.
    fn check(&self) -> Result<(), String> {
        let f: String = [&self.0.wd, "args", "kube-controller-manager"].join("/");
        if !std::path::Path::new(&f).exists() {
            return Err(format!("{} does not exist", f))
        }
        match String::from_utf8(Command::new("grep").args(vec!["-i", "use-service-account-credential", &f]).output().unwrap().stdout).unwrap().trim() {
            ss if ss.is_empty() => Err("unset".into()),
            ss if ss.ends_with("=false") => Err("is false".into()),
            _ => Ok(())
        }
    }

    /// Edit the Kubernetes Controller Manager manifest file in the /etc/kubernetes/manifests directory on the Kubernetes Master Node.
    /// Set the value of “use-service-account-credential” to “true”.
    fn fix(&self) {
        todo!()
    }
}
