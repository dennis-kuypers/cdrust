use crate::prelude::*;
use cd_aws::ec2::Ec2Instance;

pub fn select_by_user_selection(
    mut instances: Vec<Ec2Instance>,
    output: &crate::dialog::InteractiveOutput,
    message: impl AsRef<str>,
) -> anyhow::Result<Ec2Instance> {
    let short_names: Vec<String> = instances.iter().map(|i| i.to_short_string()).collect();
    let short_names_ref: Vec<_> = short_names.iter().map(AsRef::as_ref).collect();

    output
        .select_one(short_names_ref.as_slice(), message)
        .map(|i| instances.remove(i))
        .ok_or_else(|| anyhow!("No instances selected, aborting"))
}

pub fn filter_by_user_selection(
    instances: Vec<Ec2Instance>,
    output: &crate::dialog::InteractiveOutput,
    message: impl AsRef<str>,
    selected_default: bool,
) -> anyhow::Result<Vec<Ec2Instance>> {
    let short_names: Vec<String> = instances.iter().map(|i| i.to_short_string()).collect();
    let short_names_ref: Vec<_> = short_names.iter().map(AsRef::as_ref).collect();

    let selected = output.select(short_names_ref.as_slice(), message, selected_default);
    if let Some(selected_indices) = selected {
        let instances: Vec<_> = instances
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| selected_indices.contains(idx))
            .map(|(_, item)| item)
            .collect();

        if instances.is_empty() {
            bail!("No instances selected");
        }

        Ok(instances)
    } else {
        bail!("No instances selected, aborting")
    }
}
