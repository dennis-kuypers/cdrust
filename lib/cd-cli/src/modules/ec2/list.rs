use crate::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ListOpts {
    #[structopt(flatten)]
    pub select: crate::modules::ec2::Ec2SelectOpt,
}

pub async fn list(opts: ListOpts, client: &ce_aws::AwsClient, output: Output) -> Result<(), anyhow::Error> {
    let output = output.structured()?;
    let instances = super::select_instances(&opts.select, client).await?;

    output.print(instances);

    Ok(())
}
