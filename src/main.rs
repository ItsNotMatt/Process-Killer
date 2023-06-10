use winproc::{Process, Error};

#[tokio::main]
async fn main() {
    let processes = vec!["steam.exe", "nvcontainer.exe", "NVIDIA Share.exe", "NVDisplay.Container.exe", 
     "python.exe", "Unity Hub.exe", "msedge.exe", "nordvpn-service.exe", "DriverSupport.exe", "lghub_updater.exe"];

    loop{
        match get_process(&processes).await{
            Ok(p) => terminate_process(p).await,
            Err(_) => break
        };
    }
}

async fn get_process(processes: &Vec<&str>) -> Result<Process, Error>{
    Process::all()?
        .find(|p| p.name().map(|n| processes.contains(&n.as_str())).unwrap_or(false))
        .ok_or(Error::NoProcess("Can't find proc".to_string()))
}

async fn terminate_process(mut proc: Process){
    println!("Attempting to terminate {:?}", proc.name());
    if proc.is_running(){
        proc.terminate(0).expect("Couldn't terminate process ");
    }
    println!("Terminated process {:?}", proc.name());
}
