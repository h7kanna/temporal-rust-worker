use temporal_sdk::prelude::registry::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut worker = worker::worker().await.unwrap();
    worker.register_activity(
        "temporal_activity",
        into_activity_1_args_with_errors(activity::sdk_example_activity),
    );
    worker.register_wf(
        "temporal_workflow",
        into_workflow_1_args(workflow::sdk_example_workflow),
    );
    worker.run().await?;
    Ok(())
}

mod worker {
    use std::{str::FromStr, sync::Arc};
    use temporal_sdk::prelude::worker::*;

    pub(crate) async fn worker() -> Result<Worker, Box<dyn std::error::Error>> {
        let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;
        let client = server_options.connect("default", None, None).await?;
        let telemetry_options = TelemetryOptionsBuilder::default().build()?;
        let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;
        let task_queue = "example-task-queue";
        let worker_config = WorkerConfigBuilder::default()
            .namespace("default")
            .task_queue(task_queue)
            .worker_build_id("example-rust-worker")
            .build()?;
        let core_worker = init_worker(&runtime, worker_config, client)?;
        Ok(Worker::new_from_core(Arc::new(core_worker), task_queue))
    }
}

mod activity {
    use temporal_sdk::prelude::activity::*;

    #[derive(Debug, thiserror::Error)]
    #[non_exhaustive]
    pub enum Error {
        #[error(transparent)]
        Io(#[from] std::io::Error),
        #[error(transparent)]
        Any(anyhow::Error),
    }

    impl FromFailureExt for Error {
        fn from_failure(failure: Failure) -> Error {
            Error::Any(anyhow::anyhow!("{:?}", failure.message))
        }
    }

    #[derive(Default, Deserialize, Serialize, Debug, Clone)]
    pub struct ActivityInput {
        pub language: String,
        pub kind: String,
    }

    #[derive(Default, Deserialize, Serialize, Debug, Clone)]
    pub struct ActivityOutput {
        pub kind: String,
        pub platform: String,
        pub features: Vec<String>,
    }

    pub async fn sdk_example_activity(
        _ctx: ActContext,
        input: ActivityInput,
    ) -> Result<(String, ActivityOutput), Error> {
        Ok((
            format!("Workflow written in {} {}", input.kind, input.language),
            ActivityOutput {
                kind: "worker".to_string(),
                platform: "temporal".to_string(),
                features: vec![
                    "performance".to_string(),
                    "async".to_string(),
                    "type-safe".to_string(),
                    "resource-efficient".to_string(),
                ],
            },
        ))
    }
}

mod workflow {
    use super::activity::*;
    use temporal_sdk::prelude::workflow::*;

    #[derive(Default, Deserialize, Serialize, Debug, Clone)]
    pub struct WorkflowInput {
        pub code: String,
        pub kind: String,
    }

    pub async fn sdk_example_workflow(
        ctx: WfContext,
        input: WorkflowInput,
    ) -> Result<WfExitValue<ActivityOutput>, anyhow::Error> {
        let output = execute_activity_1_args_with_errors(
            &ctx,
            ActivityOptions {
                activity_id: Some("sdk_example_activity".to_string()),
                activity_type: "sdk_example_activity".to_string(),
                schedule_to_close_timeout: Some(Duration::from_secs(5)),
                ..Default::default()
            },
            sdk_example_activity,
            ActivityInput {
                language: input.code,
                kind: input.kind,
            },
        )
        .await;
        match output {
            Ok(output) => Ok(WfExitValue::Normal(output.1)),
            Err(e) => Err(anyhow::Error::from(e)),
        }
    }
}
