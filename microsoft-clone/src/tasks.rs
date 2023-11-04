use chrono::{DateTime, Utc};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf; 

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task {text, created_at}
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; //Cursor to index 0
    let tasks = match serde_json::from_reader(file){
        Ok(tasks) => tasks, 
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; //Cursor to index 0
    Ok(tasks)
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()>{
    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    // Write the modified task list back into the file
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()>{

    // Open the file
    let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(journal_path)?;

    // Read the file's contents into a vector of tasks
    let mut tasks = collect_tasks(&file)?;

    // Remove the task
    if task_position == 0 || task_position > tasks.len(){
        return Err(Error::new(ErrorKind::InvalidInput, "invalid task ID"));
    }
    tasks.remove(task_position - 1);
   
    
    // Write the modified task list back into the file
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {}

