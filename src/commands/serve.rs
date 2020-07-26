use std::fs::File;
use std::io::Write;
use std::process::Command;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;

use crate::http::proxy_server;
use crate::php::php_server;
use crate::php::php_server::PhpServerSapi;
use crate::utils::current_process_name;
use std::env;

const DEFAULT_PORT: &str = "8000";

pub(crate) fn command_config<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("serve")
        .about("Runs an HTTP server")
        .arg(
            Arg::with_name("port")
                .long("port")
                .help("The TCP port to listen to")
                .default_value(DEFAULT_PORT.as_ref())
                .takes_value(true),
        )
        .arg(
            Arg::with_name("daemon")
                .short("d")
                .long("daemon")
                .help("Run the server in the background"),
        )
        .arg(
            Arg::with_name("document-root")
                .long("document-root")
                .help("Project's document root"),
        )
        .arg(
            Arg::with_name("passthru")
                .long("passthru")
                .help("The PHP script all requests will be passed to"),
        )
}

pub(crate) fn serve(args: &ArgMatches) {
    if args.is_present("daemon") {
        serve_background(args);
    } else {
        serve_foreground(args);
    }
}

fn serve_foreground(args: &ArgMatches) {
    pretty_env_logger::init();

    info!("Starting PHP...");

    let php_server = php_server::start();

    let sapi = match php_server.sapi() {
        #[cfg(not(target_os = "windows"))]
        PhpServerSapi::FPM => "FPM",
        PhpServerSapi::CLI => "CLI",
        PhpServerSapi::CGI => "CGI",
    };
    info!("PHP started with module {}", sapi);

    info!("Starting HTTP server...");

    let default_document_root = env::current_dir().unwrap();
    let default_document_root = default_document_root.to_str().unwrap();

    let port = args.value_of("port").unwrap_or(DEFAULT_PORT);
    let document_root = args.value_of("document-root").unwrap_or(default_document_root).to_string();
    let script_filename = args.value_of("passthru").unwrap_or("index.php").to_string();

    proxy_server::start(
        port.parse::<u16>().unwrap(),
        php_server.port(),
        &document_root,
        &script_filename
    );
}

fn serve_background(args: &ArgMatches) {
    let subprocess = Command::new(current_process_name::get().as_str())
        .arg("serve")
        .arg("--port")
        .arg(args.value_of("port").unwrap_or(DEFAULT_PORT))
        .spawn()
        .expect("Failed to start server as a background process");

    let pid = subprocess.id();
    let mut file = File::create(".pid").expect("Cannot create PID file");
    file.write_all(pid.to_string().as_ref())
        .expect("Cannot write to PID file");

    info!("Background server running with PID {}", pid);
}
