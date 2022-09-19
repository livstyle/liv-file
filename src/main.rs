mod cores;
mod app;
mod api;

use skywalking::{
    logging::{logger::Logger, record::{LogRecord, RecordType}},
    reporter::grpc::GrpcReporter,
    trace::tracer::Tracer,
    metrics::{meter::Counter, metricer::Metricer},
};
use std::error::Error;
use tokio::signal;

async fn handle_request(tracer: Tracer, logger: Logger) {
    let mut ctx = tracer.create_trace_context();

    {
        // Generate an Entry Span when a request is received.
        // An Entry Span is generated only once per context.
        // Assign a variable name to guard the span not to be dropped immediately.
        let _span = ctx.create_entry_span("op1");

        // Something...

        {
            // Generates an Exit Span when executing an RPC.
            let span2 = ctx.create_exit_span("op2", "remote_peer");

            // Something...

            // Do logging.
            logger.log(
                LogRecord::new()
                    .add_tag("level", "INFO")
                    .with_tracing_context(&ctx)
                    .with_span(&span2)
                    .record_type(RecordType::Text)
                    .content("Something...")
            );

            // Auto close span2 when dropped.
        }

        // Auto close span when dropped.
    }

    // Auto report ctx when dropped.
}

async fn handle_metric(mut metricer: Metricer) {
    let counter = metricer.register(
        Counter::new("instance_trace_count")
            .add_label("region", "us-west")
            .add_label("az", "az-1"),
    );

    metricer.boot().await;

    counter.increment(10.);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    app::bootstrap();
    // Connect to skywalking oap server.
    let reporter = GrpcReporter::connect("http://0.0.0.0:11800").await?;

    // Spawn the reporting in background, with listening the graceful shutdown signal.
    let handle = reporter
        .reporting()
        .await
        .with_graceful_shutdown(async move {
            signal::ctrl_c().await.expect("failed to listen for event");
        })
        .spawn();

    let tracer = Tracer::new("service", "instance", reporter.clone());
    let logger = Logger::new("service", "instance", reporter.clone());
    let metricer = Metricer::new("service", "instance", reporter);

    handle_metric(metricer).await;

    handle_request(tracer, logger).await;

    handle.await?;

    Ok(())
}