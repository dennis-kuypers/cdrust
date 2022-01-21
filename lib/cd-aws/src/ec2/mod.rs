pub mod models;
use crate::{AwsClient, AwsClientError};
use derive_new::new;
pub use models::*;

#[derive(new, Debug)]
pub struct QueryFilter {
    pub key: String,
    pub values: Vec<String>,
}

impl From<QueryFilter> for aws_sdk_ec2::model::Filter {
    fn from(f: QueryFilter) -> Self {
        aws_sdk_ec2::model::filter::Builder::default()
            .name(f.key)
            .set_values(Some(f.values).filter(|v| !v.is_empty()))
            .build()
    }
}

pub async fn query_instances(c: &AwsClient, filters: Vec<QueryFilter>) -> Result<Vec<Ec2Instance>, AwsClientError> {
    let client = aws_sdk_ec2::client::Client::new(&c.0);
    let mut operation = client.describe_instances();

    for filter in filters {
        operation = operation.filters(filter);
    }

    let result = operation.max_results(1000).send().await.unwrap();

    let reservations: Vec<aws_sdk_ec2::model::Reservation> = result.reservations.unwrap();
    let instances: Vec<_> = reservations
        .into_iter()
        .flat_map(|r| r.instances.unwrap_or_default())
        .map(|i| i.try_into())
        .flatten()
        .collect();

    Ok(instances)
}

pub async fn start_instances(
    c: &AwsClient,
    instance_ids: Vec<String>,
) -> Result<Vec<Ec2InstanceStateChange>, AwsClientError> {
    let client = aws_sdk_ec2::client::Client::new(&c.0);
    let response = client
        .start_instances()
        .set_instance_ids(Some(instance_ids))
        .send()
        .await
        .map_err::<AwsClientError, _>(Into::into)?;

    let instance_states = response.starting_instances.unwrap();
    let results: Vec<Ec2InstanceStateChange> = instance_states.into_iter().map(Into::into).collect();
    Ok(results)
}

pub async fn stop_instances(
    c: &AwsClient,
    instance_ids: Vec<String>,
) -> Result<Vec<Ec2InstanceStateChange>, AwsClientError> {
    let client = aws_sdk_ec2::client::Client::new(&c.0);
    let response = client
        .stop_instances()
        .set_instance_ids(Some(instance_ids))
        .send()
        .await
        .map_err::<AwsClientError, _>(Into::into)?;

    let instance_states = response.stopping_instances.unwrap();
    let results: Vec<Ec2InstanceStateChange> = instance_states.into_iter().map(Into::into).collect();
    Ok(results)
}

pub async fn terminate_instances(
    c: &AwsClient,
    instance_ids: Vec<String>,
) -> Result<Vec<Ec2InstanceStateChange>, AwsClientError> {
    let client = aws_sdk_ec2::client::Client::new(&c.0);
    let response = client
        .terminate_instances()
        .set_instance_ids(Some(instance_ids))
        .send()
        .await
        .map_err::<AwsClientError, _>(Into::into)?;

    let instance_states = response.terminating_instances.unwrap();
    let results: Vec<Ec2InstanceStateChange> = instance_states.into_iter().map(Into::into).collect();
    Ok(results)
}
