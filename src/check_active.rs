use std::process::Command;
pub fn is_vpn_active() -> Result<bool, String> {
    let output = Command::new("wg")
        .arg("show")
        .output()
        .map_err(|e| format!("failed to execute wg: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("latest handshake") {
            Ok(true) //VPN AKTIV
        } else {
            Ok(false) //WG LAUEFT ABER KEIN HANDSHAKE ALSO AKTIVE VERBINDUNG
        }
    } else {
        Err(format!(
            "wg show failed with code {}",
            output.status.code().unwrap_or(-1)
        ))
    }
}
