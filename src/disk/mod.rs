use std::boxed::Box;
use std::fs::File;

pub mod boot;
mod ntfs;
mod mbr;
mod fat;
mod ext;


use ntfs::NtfsBoot;
use fat::FatBoot;
use ext::Superblock;
use mbr::Mbr;
use boot::Boot;

pub fn retrieve_boot(file: &mut File) -> Result< Box::<dyn Boot>, String>
{
    let start : u64 = 446;

    let mut ret_structure : Box::<dyn Boot> = Box::<Mbr>::new(Mbr::new(file, start));
    
    if ret_structure.verify()
    {
        return Result::Ok(ret_structure);
    }
    


    ret_structure = Box::<NtfsBoot>::new(NtfsBoot::new(file, 0));

    if ret_structure.verify()
    {
        return Result::Ok(ret_structure);
    }

    ret_structure = Box::<FatBoot>::new(FatBoot::new(file, 0));

    if ret_structure.verify()
    {
        return Result::Ok(ret_structure);
    }

    ret_structure = Box::<Superblock>::new(Superblock::new(file, 1024));

    if ret_structure.verify()
    {
        Result::Ok(ret_structure)
    }
    else
    {
        Result::Err(String::from("Could not find a working structure for this disk! Either this file is not a disk or is a disk type not supported!"))
    }

}




