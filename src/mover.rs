use crate::yee_file::YeeFile;

// Final stage in our file copier. Will move the files from their source to the destination.
pub struct Mover{}


// All left over `YeeFiles` are moved from their source to the destination. 
impl Mover{

    pub fn new() -> Self{
        Self{}
    }

    pub fn move_files(&self, files: Vec<YeeFile>){

    }

}