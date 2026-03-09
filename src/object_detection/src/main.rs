use std::sync::Arc;

use safe_drive::{context::Context, logger::Logger, node::Node, pr_info};

const NODE_NAME: &str = "object_detection";

fn main() {
    let context: Arc<Context> = Context::new().expect("Failed to create ROS2 Context");

    let node: Arc<Node> = context
        .create_node(NODE_NAME, Some("/"), Default::default())
        .inspect_err(|e| tracing::error!("Failed to create the {NODE_NAME} node. Error: {e}"))
        .expect("Node Creation");

    let logger: Arc<Logger> = Arc::new(Logger::new(NODE_NAME));
    pr_info!(logger, "The {NODE_NAME} is now online!");

    pr_info!(logger, "{NODE_NAME} has been deactivated.");
}
