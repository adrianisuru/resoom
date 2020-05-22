use clap::App;
use clap::Arg;
use log::info;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use which::which;

mod template;
use template::Template;

const PROJECT_SECTION: &str = r#"
\project{Essential Tasks ToDo List}{Dart}
{
    \item Flutter based app built for roommate who suffers from an executive 
        function disorder and poor working memory to help keep track of essential 
        tasks such as eating and bathing (Flutter, SQLite)
    \item App state is saved locally (SQLite)
}

\project{Esim}{Java}
{
    \item 3D Coulomb's Law visualization built for physics teacher (Java, OpenGL/GLSL)
    \item Allows users to place point charges in a plane and see a 3D graph of the
        electric potential at each point in the plane
}

\project{Julia Set Explorer}{Rust}
{
    \item Julia set viewer with pan and zoom capabilities (Rust, OpenGL/GLSL)
    \item Fractal is colored by OpenGL shaders to maximize performace (GLSL)
    \item Contributed a pull request to glium/glium
}

\project{Genetic Algorithm Image Evolution}{C++}
{
    \item Genetic algorithm which recreates a user provide image using simple 
        shapes (C++, openFrameworks)
}
"#;

fn main() {
    let matches = App::new("resoom")
        .arg(Arg::with_name("file").required(true))
        .arg(
            Arg::with_name("template")
                .takes_value(true)
                .long("template")
                .short("t"),
        )
        .arg(
            Arg::with_name("template_dir")
                .takes_value(true)
                .long("template-dir")
                .short("d")
                .requires("template"),
        )
        .get_matches();
    let template_dir = matches.value_of("template_dir").unwrap_or(".");
    if let Some(file) = matches.value_of("file") {
        info!("file: {}", file);
        match matches.value_of("template") {
            Some(template) => {
                let template = read_to_string(template).unwrap();
                let template = Template::new(&template);
                let source = template.build(&PROJECT_SECTION);
                write!(
                    File::create(
                        Path::new(template_dir)
                            .join(file.to_string() + ".template")
                    )
                    .unwrap(),
                    "{}",
                    source
                );
                compile(
                    check_path().expect("compiler not found"),
                    &(file.to_string() + ".template"),
                    template_dir,
                );
            }
            None => {
                compile(
                    check_path().expect("compiler not found"),
                    &file,
                    template_dir,
                );
            }
        }
    }
}

/// Constructs a new `Command` from the first program from `compilers.txt` found
/// by `which`
fn check_path() -> Option<Command> {
    match include_str!("compilers.txt")
        .lines()
        .find(|c| which(c).is_ok())
    {
        Some(c) => Some(Command::new(c)),
        None => {
            info!("no compiler found locally");
            None
        }
    }
}

fn compile(mut compiler: Command, source: &ToString, current_dir: &str) {
    compiler
        .current_dir(current_dir)
        .arg(source.to_string())
        .spawn()
        .expect("error executing")
        .wait();
}
