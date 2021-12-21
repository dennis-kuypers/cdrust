use crate::prelude::*;
use cd_aws::ec2::{Ec2Instance, QueryFilter};

#[derive(StructOpt, Debug)]
pub struct Ec2SelectOpt {
    /// Filter instances
    /// - start with `i-` to filter (starts_with) match on instance id
    /// - anything else will be matched (contains) on the tag `Name`
    pub query: Option<String>,

    /// Additional raw filters `key=value` or `key=value1,value2,...`
    /// Can be specified multiple times (logical AND)
    /// https://docs.rs/aws-sdk-ec2/0.0.26-alpha/aws_sdk_ec2/input/struct.DescribeInstancesInput.html#structfield.filters
    #[structopt(long)]
    pub filter: Option<Vec<String>>,
}

pub async fn select_instances(
    opts: &Ec2SelectOpt,
    client: &cd_aws::AwsClient,
) -> anyhow::Result<Vec<cd_aws::ec2::Ec2Instance>> {
    let filters: Vec<_> = opts
        .filter
        .as_ref()
        .unwrap_or(&vec![])
        .iter()
        .map(|f| parse_query_filter(f.as_str()))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let instances = cd_aws::ec2::query_instances(&client, filters).await?;
    let user_query = Ec2InstanceFilter::new(opts.query.clone());
    let instances: Vec<_> = instances.into_iter().filter(|i| user_query.filter(i)).collect();
    Ok(instances)
}

fn parse_query_filter(s: &str) -> anyhow::Result<QueryFilter> {
    let (key, values) = s
        .split_once('=')
        .ok_or_else(|| anyhow!("{} is not a valid filter", s))?;

    let key = key.to_owned();
    let values: Vec<String> = values.split(',').map(|s| s.to_owned()).collect();

    Ok(QueryFilter { key, values })
}

#[derive(Debug, Eq, PartialEq)]
enum Ec2InstanceFilterKind {
    None,
    InstanceId(String),
    Text(String),
}

struct Ec2InstanceFilter(Ec2InstanceFilterKind);
impl Ec2InstanceFilter {
    pub fn new(f: Option<String>) -> Self {
        let kind = match f {
            None => Ec2InstanceFilterKind::None,
            Some(instance) if instance.starts_with("i-") => Ec2InstanceFilterKind::InstanceId(instance),
            Some(query) => Ec2InstanceFilterKind::Text(query),
        };

        Self(kind)
    }

    pub fn filter(&self, i: &Ec2Instance) -> bool {
        match &self.0 {
            Ec2InstanceFilterKind::None => true,
            Ec2InstanceFilterKind::InstanceId(id) => i.id.starts_with(id),
            Ec2InstanceFilterKind::Text(t) => i.tags.get("Name").map(|name| name.contains(t)).unwrap_or_default(),
        }
    }
}
