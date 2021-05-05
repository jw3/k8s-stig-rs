use std::process::Command;

use crate::Rule;

/// Kubernetes API Server must disable basic authentication to protect information in transit.
/// Kubernetes basic authentication sends and receives request containing username, uid, groups,
/// and other fields over a clear text HTTP communication. Basic authentication does not provide any
/// security mechanisms using encryption standards. PKI certificate-based authentication must be set
/// over a secure channel to ensure confidentiality and integrity. Basic authentication must not be
/// set in the manifest file.
/// CNTR-K8-002620
/// Category I
impl<'a> Rule for crate::K2620<'a> {
    type ErrType = String;

    fn id(&self) -> String {
        "CNTR-K8-002620".into()
    }

    /// Change to the /etc/kubernetes/manifests/ directory on the Kubernetes Master Node. Run the
    /// command: grep -i basic-auth-file *
    /// If “basic-auth-file” is set in the Kubernetes API server manifest file, this is a finding.
    fn check(&self) -> Result<(), Self::ErrType> {
        let f: String = [&self.0.wd, "args", "kube-apiserver"].join("/");
        if !std::path::Path::new(&f).exists() {
            return Err(format!("{} does not exist", f))
        }
        match String::from_utf8(Command::new("grep").args(vec!["-i", "basic-auth-file", &f]).output().unwrap().stdout).unwrap().trim() {
            ss if !ss.is_empty() => Err("basic-auth-file is set".into()),
            _ => Ok(())
        }
    }

    /// Edit the Kubernetes API Server manifest file in the /etc/kubernetes/manifests directory on
    /// the Kubernetes Master Node. Remove the setting “--basic-auth-file”.
    fn fix(&self) {
        todo!()
    }
}
