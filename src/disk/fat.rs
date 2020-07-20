/**
 * File: fat.rs
 * Purpose: Provides support for browsing early Windows File Systems
 */

use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;

use crate::disk::boot::Boot;


pub struct FatBoot
{
    //jmp: [u8;3],
    oem: [char;8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    table_count: u8,
    directory_entries: u16,
    sector_count_s: u16, 
    media_type: u8,
    sectors_per_fat: u16,
    sectors_per_track: u16,
    heads_on_media: u16,
    hidden_sectors: u32,
    sector_count_l: u32
}

pub struct FatBoot12_16
{
    drive_number: u8,
    nt_flags: u8,
    signature: u8,
    volumne_id: u32,
    volumne_label: [char;11],
    system_id: [char;8]
}

pub struct FatBoot32
{
    sectors_per_fat: u32,
    flags: u16,
    fat_version: u16,
    root_cluster: u32,
    fs_info_sector: u16,
    backup_sector: u16,
    zeros: [u8;12],
    drive_num: u8,
    flags_nt: u8,
    signature: u8,
    volumne_id: u32,
    volumne_label: [char;11],
    system_id: [char;8]
}

impl FatBoot
{
    pub fn new(file: &mut File, start:u64) -> FatBoot {
        let mut buffer = [0; 36];

        file.seek(SeekFrom::Start(start));

        file.read(&mut buffer);

        let mut oem_str: [char ; 8] = ['\0'; 8]; 

        for (oem_element, source_element) in oem_str.iter_mut().zip(buffer[3..11].iter())
        {
            *oem_element = (*source_element) as char;
        }

        FatBoot {
            // jmp: clone_into_array(&buffer[0..2]),
            oem: oem_str,
            bytes_per_sector: buffer[11] as u16 + ((buffer[12] as u16) << 8),
		    sectors_per_cluster: buffer[13],
		    reserved_sectors: buffer[14] as u16,// + (buffer[15] as u16) << 8,
		    table_count: buffer[16],
		    directory_entries: buffer[17] as u16 + ((buffer[18] as u16) << 8),
		    sector_count_s: buffer[19] as u16 + ((buffer[20] as u16) << 8),
		    media_type: buffer[21],
		    sectors_per_fat: buffer[22] as u16 + ((buffer[23] as u16) << 8),
		    sectors_per_track: buffer[24] as u16 + ((buffer[25] as u16) << 8),
		    heads_on_media: buffer[26] as u16 + ((buffer[27] as u16) << 8),
		    hidden_sectors: ((buffer[28] as u32)) 
		    	+ ((buffer[29] as u32) << 8)
		    	+ ((buffer[30] as u32) << 16)
			    + ((buffer[31] as u32) << 24),
            sector_count_l: ((buffer[32] as u32)) 
		    	+ ((buffer[33] as u32) << 8)
		    	+ ((buffer[34] as u32) << 16)
		    	+ ((buffer[35] as u32) << 24),	
        }
    }
}

impl Boot for FatBoot
{
    fn print_boot_level(&self)
    {  
        println!("Printing boot Structure FAT Boot Record!");
        // println!("oem_str: {}", String::from(self.oem));
        println!("Bytes Per Sector: {}", self.bytes_per_sector);
        println!("Sectors Per Cluster: {}", self.sectors_per_cluster);
        println!("Reserved Sectors: {}", self.reserved_sectors);
        println!("Table Count: {}", self.table_count);
        println!("Root Entry Count: {}", self.directory_entries);
        println!("Sector Count (small): {}", self.sector_count_s);
        println!("Media Type: {}", self.media_type);
        println!("Sectors Per Table: {}", self.sectors_per_fat);
        println!("Sectors per Track: {}", self.sectors_per_track);
        println!("Head Count: {}", self.heads_on_media);
        println!("Hidden Sectors: {}", self.hidden_sectors);
        println!("Sector Count (large): {}", self.sector_count_l);
    }

    fn verify(&self) -> bool
    {
        (self.sector_count_l == 0) != (self.sector_count_s == 0) 
    }
}