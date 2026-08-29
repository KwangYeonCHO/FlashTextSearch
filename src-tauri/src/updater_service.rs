use std::env;
use std::fs;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub size: u64,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GhReleaseInfo {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub published_at: String,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckResult {
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_title: String,
    pub release_notes: String,
    pub published_at: String,
}

/// 自动更新服务：基于 GitHub Release 与 Windows 进程替换机制
pub struct UpdaterService;

impl UpdaterService {
    /// 检查 GitHub Release 最新版本（超快轻量无阻塞）
    pub fn check_update() -> Result<UpdateCheckResult, String> {
        let current_ver = env!("CARGO_PKG_VERSION");

        // 优先使用快速轻量的 curl 请求 GitHub 公开 API (带超时控制与无窗口标志)
        let release: GhReleaseInfo = match Self::fetch_release_via_curl() {
            Ok(info) => info,
            Err(_) => {
                // 后备尝试 gh CLI
                Self::fetch_release_via_gh()?
            }
        };

        let clean_tag = release.tag_name.trim_start_matches('v').trim();
        let clean_cur = current_ver.trim_start_matches('v').trim();

        let has_update = Self::is_version_newer(clean_cur, clean_tag);

        Ok(UpdateCheckResult {
            has_update,
            current_version: format!("v{}", clean_cur),
            latest_version: release.tag_name,
            release_title: release.name,
            release_notes: release.body,
            published_at: release.published_at,
        })
    }

    /// 使用极速轻量的 curl 命令请求 GitHub Release API (带 2s 连接超时与 3s 最大总耗时)
    fn fetch_release_via_curl() -> Result<GhReleaseInfo, String> {
        let url = "https://api.github.com/repos/KwangYeonCHO/FlashTextSearch/releases/latest";
        let mut cmd = Command::new("curl");
        cmd.args(&[
            "-s",
            "--connect-timeout",
            "2",
            "--max-time",
            "3",
            "-H",
            "User-Agent: FlashTextSearch-App",
            "-H",
            "Accept: application/vnd.github.v3+json",
            url,
        ]);

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let output = cmd.output().map_err(|e| format!("请求 GitHub API 失败: {}", e))?;

        if !output.status.success() || output.stdout.is_empty() {
            return Err("无法连接到 GitHub Release 服务".to_string());
        }

        #[derive(Deserialize)]
        struct ApiRelease {
            tag_name: String,
            name: Option<String>,
            body: Option<String>,
            published_at: Option<String>,
        }

        let api_rel: ApiRelease = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("解析 GitHub API 响应失败: {}", e))?;

        Ok(GhReleaseInfo {
            tag_name: api_rel.tag_name.clone(),
            name: api_rel.name.unwrap_or(api_rel.tag_name),
            body: api_rel.body.unwrap_or_default(),
            published_at: api_rel.published_at.unwrap_or_default(),
            assets: vec![],
        })
    }

    /// 后备使用 gh CLI
    fn fetch_release_via_gh() -> Result<GhReleaseInfo, String> {
        let mut cmd = Command::new("gh");
        cmd.args(&[
            "release",
            "view",
            "--repo",
            "KwangYeonCHO/FlashTextSearch",
            "--json",
            "tagName,name,body,publishedAt,assets",
        ]);

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let output = cmd.output().map_err(|e| format!("执行 gh 失败: {}", e))?;
        if !output.status.success() {
            return Err("GitHub CLI 未就绪".to_string());
        }

        serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("解析 Release 信息失败: {}", e))
    }

    /// 下载最新 Release 附件并自动替换更新并重启
    pub fn download_and_install_update(tag_name: &str) -> Result<(), String> {
        let current_exe = env::current_exe().map_err(|e| format!("获取当前可执行文件路径失败: {}", e))?;
        let temp_dir = env::temp_dir().join("flashtext_update");
        let _ = fs::create_dir_all(&temp_dir);

        let target_download_file = temp_dir.join("FlashTextSearch.exe");
        let tag = if tag_name.starts_with('v') { tag_name.to_string() } else { format!("v{}", tag_name) };

        // 下载可执行文件
        let direct_download_url = format!(
            "https://github.com/KwangYeonCHO/FlashTextSearch/releases/download/{}/FlashTextSearch.exe",
            tag
        );

        // 使用 curl 下载
        let download_output = Command::new("curl")
            .args(&[
                "-L",
                "-s",
                "-o",
                target_download_file.to_str().unwrap_or("FlashTextSearch.exe"),
                &direct_download_url,
            ])
            .output()
            .map_err(|e| format!("下载 Release 失败: {}", e))?;

        if !download_output.status.success() || !target_download_file.exists() {
            return Err("下载更新程序失败".to_string());
        }

        // 生成 Windows 更新替换脚本
        let script_path = temp_dir.join("flashtext_updater.bat");
        let current_pid = std::process::id();

        let bat_content = format!(
            "@echo off\r\n\
            timeout /t 1 /nobreak > nul\r\n\
            taskkill /F /PID {} > nul 2>&1\r\n\
            timeout /t 1 /nobreak > nul\r\n\
            copy /Y \"{}\" \"{}\" > nul\r\n\
            start \"\" \"{}\"\r\n\
            del \"%~f0\" > nul 2>&1\r\n\
            exit\r\n",
            current_pid,
            target_download_file.display(),
            current_exe.display(),
            current_exe.display()
        );

        fs::write(&script_path, bat_content).map_err(|e| format!("写入更新脚本失败: {}", e))?;

        // 启动更新脚本并退出当前进程
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            Command::new("cmd")
                .args(&["/C", script_path.to_str().unwrap_or("flashtext_updater.bat")])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .map_err(|e| format!("启动自动更新脚本失败: {}", e))?;
        }

        std::process::exit(0);
    }

    /// 语义化版本比较 (若 latest > current 则返回 true)
    fn is_version_newer(current: &str, latest: &str) -> bool {
        let cur_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();
        let lat_parts: Vec<u32> = latest.split('.').filter_map(|s| s.parse().ok()).collect();

        for i in 0..cur_parts.len().max(lat_parts.len()) {
            let c = cur_parts.get(i).cloned().unwrap_or(0);
            let l = lat_parts.get(i).cloned().unwrap_or(0);
            if l > c {
                return true;
            } else if l < c {
                return false;
            }
        }
        false
    }
}
