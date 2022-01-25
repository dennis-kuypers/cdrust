pub async fn select_story(
    client: &cd_pivotaltracker::Client,
    project_id: u64,
    filter: &str,
    dialog: &impl super::dialog::DialogProvider,
) -> anyhow::Result<cd_pivotaltracker::Story> {
    anyhow::bail!("meh");
}

pub fn parse_pivotal_story_id(s: &str) -> Result<u64, String> {
    // we consider everything the user may throw at us

    // 1. a u64
    if let Ok(id) = s.parse::<u64>() {
        return Ok(id);
    }

    // 2. a u64 prefixed with '#'
    if let Ok(id) = s.trim_start_matches('#').parse::<u64>() {
        return Ok(id);
    }

    // 3. a pivotal url containing a story id
    if let Ok(url) = url::Url::parse(s) {
        if let Some(mut segments) = url.path_segments() {
            // we could check the hostname but that probably doesnt help us any way
            // ensure that path is "/story/show/<ID>" though...
            if segments.next() == Some("story") && segments.next() == Some("show") {
                if let Some(id) = segments.next().and_then(|s| s.parse::<u64>().ok()) {
                    return Ok(id);
                }
            }
        }
    }
    Err("Can not parse ticket id".to_string())
}
