use reqwest::blocking::Client;
use serde_json::{json, Value};

pub struct GitHub {
    client: Client,
    token: String,
    owner: String,
    repo: String,
}

impl GitHub {
    pub fn new(token: String, owner: String, repo: String) -> Self {
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .expect("Could not build HTTP client");
        Self {
            client,
            token,
            owner,
            repo,
        }
    }

    pub fn trigger_workflow(&self, workflow_file: &str) -> Result<(), String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/workflows/{}/dispatches",
            self.owner, self.repo, workflow_file
        );

        let res = self.client
            .post(&url)
            .bearer_auth(&self.token)
            .header("User-Agent", "bezalel")
            .header("Accept", "application/vnd.github+json")
            .json(&json!({ "ref": "main" }))
            .send()
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("GitHub API error: {}", res.status()))
        }
    }

    pub fn get_latest_run_id(&self, workflow_file: &str) -> Result<u64, String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/workflows/{}/runs?per_page=1",
            self.owner, self.repo, workflow_file
        );

        let res: Value = self.client
            .get(&url)
            .bearer_auth(&self.token)
            .header("User-Agent", "bezalel")
            .header("Accept", "application/vnd.github+json")
            .send()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;

        res["workflow_runs"][0]["id"]
            .as_u64()
            .ok_or("Could not get run ID".to_string())
    }

    pub fn poll_run(&self, run_id: u64) -> Result<String, String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/runs/{}",
            self.owner, self.repo, run_id
        );

        let res: Value = self.client
            .get(&url)
            .bearer_auth(&self.token)
            .header("User-Agent", "bezalel")
            .header("Accept", "application/vnd.github+json")
            .send()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;

        res["status"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or("Could not get run status".to_string())
    }

    pub fn download_artifacts(&self, run_id: u64, dest: &str) -> Result<(), String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/actions/runs/{}/artifacts",
            self.owner, self.repo, run_id
        );

        let res: Value = self.client
            .get(&url)
            .bearer_auth(&self.token)
            .header("User-Agent", "bezalel")
            .header("Accept", "application/vnd.github+json")
            .send()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;

        println!("  artifact response: {}", res);

        let artifacts = res["artifacts"]
            .as_array()
            .ok_or("No artifacts found")?;

        std::fs::create_dir_all(dest).map_err(|e| e.to_string())?;

        for artifact in artifacts {
            let name = artifact["name"].as_str().unwrap_or("unknown");
            let download_url = artifact["archive_download_url"]
                .as_str()
                .ok_or("No download URL")?;

            println!("Downloading {}...", name);

            let mut response = self.client
                .get(download_url)
                .bearer_auth(&self.token)
                .header("User-Agent", "bezalel")
                .send()
                .map_err(|e| e.to_string())?;

            println!("  download status: {}", response.status());

            let path = format!("{}/{}.zip", dest, name);
            let mut file = std::fs::File::create(&path)
                .map_err(|e| e.to_string())?;

            std::io::copy(&mut response, &mut file)
                .map_err(|e| e.to_string())?;

            println!("✓ saved to {}", path);
        }

        Ok(())
    }
}