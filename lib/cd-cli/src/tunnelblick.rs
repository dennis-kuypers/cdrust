pub fn wait_for_connection<F>(wait: std::time::Duration, retries: u32, f: F) -> anyhow::Result<bool>
where
    F: Fn(Vec<cd_tunnelblick::Vpn>) -> anyhow::Result<bool>,
{
    for _ in 1..=retries {
        let status = cd_tunnelblick::get_status()?;
        match f(status) {
            Ok(false) => std::thread::sleep(wait),
            failure_or_success => return failure_or_success,
        }
    }

    Ok(false)
}
