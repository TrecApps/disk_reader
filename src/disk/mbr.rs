/**
 * File: mbr.rs
 * Purpose: Provides structures and functions necessary for browsing the master boot record.
 */

use std::fs::File;

use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;

use crate::disk::boot::Boot;





pub struct Partition {
     
     bootable: bool,
     chs_start: u32,
     partition_type: u8,
     chs_end: u32,
     lba: u32,
     size: u32
     
 }

impl Partition{

    pub fn new(file: &mut File, start: u64) -> Partition{
        let mut buffer = [0; 16];

        file.seek(SeekFrom::Start(start));

        file.read(&mut buffer);

        Partition {
            bootable: buffer[0] == 0x80,
            chs_start: ((buffer[3] as u32) << 16) +
                        ((buffer[2] as u32) << 8) +
                        (buffer[1] as u32),
            partition_type: buffer[4],
            chs_end: ((buffer[7] as u32) << 16) +
                        ((buffer[6] as u32) << 8) +
                        (buffer[5] as u32),
            lba: ((buffer[11] as u32) << 24) +
                    ((buffer[10] as u32) << 16) +
                    ((buffer[9] as u32) << 8) +
                    (buffer[8] as u32),
            size:  ((buffer[15] as u32) << 24) +
                    ((buffer[14] as u32) << 16) +
                    ((buffer[13] as u32) << 8) +
                    (buffer[12] as u32)
        }
    }

 }


 pub struct Mbr 
 {
    partition_1: Partition,
    partition_2: Partition,
    partition_3: Partition,
    partition_4: Partition,
    signature: u16
 }

 impl Mbr{
    pub fn new(file: &mut File, start: u64) -> Mbr {

        

        file.seek(SeekFrom::Start(start + (16 * 4)));

        let mut buffer = [0;2];

        file.read(&mut buffer);

        let mut signature = u16::from_ne_bytes(buffer);

        Mbr {
            partition_1 : Partition::new(file, start),
            partition_2 : Partition::new(file, start + 16),
            partition_3 : Partition::new(file, start + 32),
            partition_4 : Partition::new(file, start + 48),
            signature: signature
        }

    }
 }

 impl Boot for Partition
{
    fn print_boot_level(&self)
    {
        println!("Printing boot Structure Master Boot Record!");
        println!("bootable: {}", self.bootable);
        println!("chs_start: {}", self.chs_start);
        println!("partition_type: {:X}", self.partition_type);
        println!("chs_end: {}", self.chs_end);
        println!("lba: {}", self.lba);
        println!("size: {}", self.size);
    }

    fn verify(&self) -> bool
    {
        true
    }
}

impl Boot for Mbr{
    fn print_boot_level(&self)
    {
        println!("First Partition!\n");
        self.partition_1.print_boot_level();

        println!("Second Partition!\n");
        self.partition_2.print_boot_level();

        println!("Third Partition!\n");
        self.partition_3.print_boot_level();

        println!("Fourth Partition!\n");
        self.partition_4.print_boot_level();
    }

    fn verify(&self) -> bool
    {
        println!("Mbr Verification: signature = {:X}", self.signature);
        self.signature == 0x55aa
    }
}