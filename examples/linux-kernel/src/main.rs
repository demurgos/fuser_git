use fuser_git::fuser::MountOption;
use fuser_git::opentelemetry::OpentelemetryFuser;
use fuser_git::{fuser, FuserGit};
use opentelemetry::trace::{Span, Tracer, TracerProvider};
use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use std::thread::sleep;
use std::time::Duration;

pub fn main() {
    // Setup LoggerProvider with a stdout exporter
    let exporter = opentelemetry_stdout::SpanExporterBuilder::default()
        // uncomment the below lines to pretty print output.
        // .with_encoder(|writer, data|
        //    Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
        .build();
    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_config(
            opentelemetry_sdk::trace::Config::default().with_resource(Resource::new(vec![
                KeyValue::new(SERVICE_NAME, "fuser-git-linux-kernel"),
            ])),
        )
        .with_simple_exporter(exporter)
        .build();

    let fs = FuserGit::new();
    let fs = OpentelemetryFuser::new_with_provider(fs, &tracer_provider);

    let t = tracer_provider.tracer("test");
    let mut s = t.start("hey");
    let mount = std::env::current_dir().unwrap().join("kernel");
    eprintln!("mounting linux kernel at: {:?}", &mount);
    std::fs::create_dir_all(&mount).unwrap();
    s.end();
    fuser::mount2(
        fs,
        mount,
        &[
            MountOption::RO,
            MountOption::FSName("fuser_git".to_string()),
            MountOption::AutoUnmount,
        ],
    )
    .unwrap();
    eprintln!("Hello!");
    sleep(Duration::from_secs(60));
    eprintln!("Bye!");
}
