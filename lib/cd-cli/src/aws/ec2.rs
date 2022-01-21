use std::str::FromStr;
use crate::prelude::*;
use cd_aws::ec2::{Ec2Instance, QueryFilter};

#[derive(StructOpt, Debug)]
pub struct Ec2SelectOpt {
    /// Additional raw filters `key=value` or `key=value1,value2,...`
    /// Can be specified multiple times (logical AND)
    /// https://docs.rs/aws-sdk-ec2/0.0.26-alpha/aws_sdk_ec2/input/struct.DescribeInstancesInput.html#structfield.filters
    #[structopt(long)]
    pub filter: Option<Vec<String>>,

    /// Filter instances
    /// - start with `i-` to filter (starts_with) match on instance id
    /// - anything else will be matched (contains) on the tag `Name`
    pub query: Vec<String>,
}

impl Ec2SelectOpt {
    pub fn has_no_filters(&self) -> bool {
        self.query.is_empty() && self.filter.as_ref().map(|f| f.is_empty()).unwrap_or(true)
    }
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

    let instances = cd_aws::ec2::query_instances(client, filters).await?;
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
    InstanceId(String),
    Text(String),
}

impl FromStr for Ec2InstanceFilterKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(()),
            instance if instance.starts_with("i-") => Ok(Ec2InstanceFilterKind::InstanceId(instance.to_string())),
            query => Ok(Ec2InstanceFilterKind::Text(query.to_string())),
        }
    }
}

struct Ec2InstanceFilter(Vec<Ec2InstanceFilterKind>);

impl Ec2InstanceFilter {
    pub fn new(f: Vec<String>) -> Self {
        let filters: Vec<Ec2InstanceFilterKind> = f.into_iter()
            .map(|s| s.parse())
            .flatten()
            .collect();

        Self(filters)
    }

    pub fn filter(&self, i: &Ec2Instance) -> bool {
        self.0.iter().all(|filter| {
            match filter {
                Ec2InstanceFilterKind::InstanceId(id) => i.id.starts_with(id),
                Ec2InstanceFilterKind::Text(t) => i.tags.get("Name").map(|name| name.contains(t)).unwrap_or_default(),
            }
        })
    }
}
