/**
 * File: ntfs.rs
 * Purpose: Provides support for browsing Windows File Systems
 */

use std::fs::File;


use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;

use crate::helper::slice_to_u32;
use crate::helper::slice_to_u64;

use crate::disk::boot::Boot;


pub struct NtfsBoot
{
    //jmp: [u8;3],
    oem_sys: [char;8],
    bytes_per_sector: u16,
    sector_per_cluster: u8,
    res_sector: u16,
    table_count: u8,
    root_entry_count: u16,
    sector_count_s: u16,
    media_type: u8,
    sectors_p_table: u16,
    sectors_p_track: u16,
    heads: u16,
    hidden_sectors: u32,
    sector_count_l: u32,
    reserved: u32,
    sector_count_xl: u64,

    mast_table_cluster_1: u64,
    mast_table_cluster_2: u64,
    clusters_p_record: u8,
    reserved_2: [u8;3],
    serial_: u64,
    checksum: u32
}

impl NtfsBoot
{
    pub fn new(file: &mut File, start: u64) -> NtfsBoot
    {
        let mut buffer = [0; 80];


        file.seek(SeekFrom::Start(start));

        file.read(&mut buffer);

        let mut oem_str: [char ; 8] = ['\0'; 8]; 

        for (oem_element, source_element) in oem_str.iter_mut().zip(buffer[3..11].iter())
        {
            *oem_element = (*source_element) as char;
        }

        NtfsBoot {
            // jmp: clone_into_array(&buffer[0..2]),
            oem_sys: oem_str,
            bytes_per_sector: buffer[11] as u16 + ((buffer[12] as u16) << 8),
		    sector_per_cluster: buffer[13],
		    res_sector: buffer[14] as u16,// + (buffer[15] as u16) << 8,
		    table_count: buffer[16],
		    root_entry_count: buffer[17] as u16 + ((buffer[18] as u16) << 8),
		    sector_count_s: buffer[19] as u16 + ((buffer[20] as u16) << 8),
		    media_type: buffer[21],
		    sectors_p_table: buffer[22] as u16 + ((buffer[23] as u16) << 8),
		    sectors_p_track: buffer[24] as u16 + ((buffer[25] as u16) << 8),
		    heads: buffer[26] as u16 + (buffer[27] as u16) << 8,
		    hidden_sectors: ((buffer[28] as u32)) 
		    	+ ((buffer[29] as u32) << 8)
		    	+ ((buffer[30] as u32) << 16)
			    + ((buffer[31] as u32) << 24),
            sector_count_l: ((buffer[32] as u32)) 
		    	+ ((buffer[33] as u32) << 8)
		    	+ ((buffer[34] as u32) << 16)
                + ((buffer[35] as u32) << 24),
            reserved: slice_to_u32(&buffer[36..40], true),
            sector_count_xl: slice_to_u64(&buffer[40..48], true),

            mast_table_cluster_1: slice_to_u64(&buffer[48..56],true),
            mast_table_cluster_2: slice_to_u64(&buffer[56..64],true),
            clusters_p_record: buffer[64],
            reserved_2: [buffer[65], buffer[66], buffer[67]] ,
            serial_: slice_to_u64(&buffer[68..76], true),
            checksum: slice_to_u32(&buffer[76..80], true)
        }
    }
}


impl Boot for NtfsBoot
{
    fn print_boot_level(&self)
    {
        println!("Printing boot Structure NTFS Boot Record!");
        println!("Bytes Per Sector: {}", self.bytes_per_sector);
        println!("Sectors Per Cluster: {}", self.sector_per_cluster);
        println!("Reserved Sectors: {}", self.res_sector);
        println!("Table Count: {}", self.table_count);
        println!("Root Entry Count: {}", self.root_entry_count);
        println!("Sector Count (small): {}", self.sector_count_s);
        println!("Media Type: {}", self.media_type);
        println!("Sectors Per Table: {}", self.sectors_p_table);
        println!("Sectors per Track: {}", self.sectors_p_track);
        println!("Head Count: {}", self.heads);
        println!("Hidden Sectors: {}", self.hidden_sectors);
        println!("Sector Count (large): {}", self.sector_count_l);
        println!("Sector Count (x-large): {}", self.sector_count_xl);
        println!("Master Table Cluster 1: {}", self.mast_table_cluster_1);
        println!("Master Table Cluster 2: {}", self.mast_table_cluster_2);

        println!("Clusters Per Record: {}", self.clusters_p_record);
        println!("Serial: {}", self.serial_);
        println!("Checksum: {}\n", self.checksum);
    }

    fn verify(&self) -> bool
    {
        self.oem_sys[0] == 'N' &&
        self.oem_sys[0] == 'T' &&
        self.oem_sys[0] == 'F' &&
        self.oem_sys[0] == 'S'
        
    }
}