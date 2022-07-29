#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use aws::Aws;
use color_eyre::Result;
use gcloud::Gcloud;
use serde::Serialize;

#[derive(Serialize)]
pub struct AwsPolicy {
    path: String,
    policy_name: String,
    policy_id: String,
    arn: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut gcloud = Gcloud::iam().roles().list().format("json");
    let output = gcloud.output()?;
    std::fs::write("rolescraper_gcp.json", output.stdout)?;

    let aws = Aws::new(
        &std::env::var("AWS_ACCESS_KEY_ID")?,
        &std::env::var("AWS_SECRET_ACCESS_KEY")?,
    );

    let mut all_aws_policies = vec![];

    let mut aws_policies = aws.list_policies(None, None, Some(1000)).await?;
    if let Some(policies) = aws_policies.policies {
        for policy in policies {
            if let Some(arn) = &policy.arn {
                if arn.starts_with("arn:aws:iam::aws:policy") && policy.is_attachable {
                    all_aws_policies.push(AwsPolicy {
                        path: policy.path.unwrap(),
                        policy_name: policy.policy_name.unwrap(),
                        policy_id: policy.policy_id.unwrap(),
                        arn: arn.clone(),
                    });
                }
            }
        }
    }

    while aws_policies.is_truncated {
        aws_policies = aws
            .list_policies(None, aws_policies.marker, Some(1000))
            .await?;

        if let Some(policies) = aws_policies.policies {
            for policy in policies {
                if let Some(arn) = &policy.arn {
                    if arn.starts_with("arn:aws:iam::aws:policy") && policy.is_attachable {
                        all_aws_policies.push(AwsPolicy {
                            path: policy.path.unwrap(),
                            policy_name: policy.policy_name.unwrap(),
                            policy_id: policy.policy_id.unwrap(),
                            arn: arn.clone(),
                        });
                    }
                }
            }
        }
    }

    std::fs::write(
        "rolescraper_aws.json",
        serde_json::to_string_pretty(&all_aws_policies)?,
    )?;

    Ok(())
}
