use clap::Args;

#[derive(Args)]
pub struct ProcessArgs {
    /// filename of the document to be processed
    #[arg(short, long)]
    pub filename: Option<String>,

    /// filename of the document to be processed
    #[arg(short, long, default_value_t = String::from("/.fessora"))]
    output: String,
}

fn download_pdf() {

}

fn save_pdf() {

}

fn process_pdf() {

}