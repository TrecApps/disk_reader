/**
 * File: ext.rs
 * Purpose: Provides support for browsing Linux File Systems
 */

use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;

use crate::helper::slice_to_u32;
use crate::helper::slice_to_u16;

use crate::disk::boot::Boot;

pub struct Superblock
{
    inode_count: u32,
    block_count: u32,
    superuser_blocks: u32,
    unallocated_blocks: u32,
    allocated_blocks: u32,
    superblock_loc: u32,
    log_2_block_size: u32,
    log_2_frag_size: u32,
    blocks_per_group: u32,
    fragments_per_group: u32,
    inodes_per_group: u32,
    last_mount_time: u32,
    last_written_time: u32,
    mount_count_check: u16,
    mount_limit_check: u16,
    signature: u16,
    state: u16,
    state_response: u16,
    minor_version:u16,
    last_check: u32,
    force_check: u32,
    os_id: u32,
    major_version: u32,
    user_id: u16,
    groupd_id: u16
}

struct SuperblockExt
{
    first_inode: u32,
    inode_size: u16,
    block_group: u16,
    opt_features: u32,
    req_features: u32,
    write_needed_features: u32,
    fs_id: [u8;16],
    vol_name: [char;16],
    vol_path: [char;64],
    comp_algorithm: u32,
    blocks_for_files: u8,
    blocks_for_directories: u8,
    unused: u16,
    journal_id: [char;16],
    journal_inode: u32,
    journal_device: u32,
    head_orphan: u32
}

impl Superblock
{
    pub fn new(file: &mut File, start:u64) -> Superblock {
        let mut buffer = [0; 84];

        file.seek(SeekFrom::Start(start));

        file.read(&mut buffer);

        Superblock
        {
            inode_count: slice_to_u32(&buffer[3..0], true),
            block_count: slice_to_u32(&buffer[7..4], true),
            superuser_blocks: slice_to_u32(&buffer[11..8], true),
            unallocated_blocks: slice_to_u32(&buffer[15..12], true),
            allocated_blocks: slice_to_u32(&buffer[19..16], true),
            superblock_loc: slice_to_u32(&buffer[23..20], true),
            log_2_block_size: slice_to_u32(&buffer[27..24], true),
            log_2_frag_size: slice_to_u32(&buffer[31..28], true),
            blocks_per_group: slice_to_u32(&buffer[35..32], true),
            fragments_per_group: slice_to_u32(&buffer[39..36], true),
            inodes_per_group: slice_to_u32(&buffer[43..40], true),
            last_mount_time: slice_to_u32(&buffer[47..44], true),
            last_written_time: slice_to_u32(&buffer[51..48], true),
            mount_count_check: slice_to_u16(&buffer[53..52], true),
            mount_limit_check: slice_to_u16(&buffer[55..54], true),
            signature: slice_to_u16(&buffer[57..56], true),
            state: slice_to_u16(&buffer[59..58], true),
            state_response: slice_to_u16(&buffer[61..60], true),
            minor_version:slice_to_u16(&buffer[63..62], true),
            last_check: slice_to_u32(&buffer[67..64], true),
            force_check: slice_to_u32(&buffer[71..68], true),
            os_id: slice_to_u32(&buffer[75..72], true),
            major_version: slice_to_u32(&buffer[79..76], true),
            user_id: slice_to_u16(&buffer[81..80], true),
            groupd_id: slice_to_u16(&buffer[83..82], true)
        }
    }
}


impl Boot for Superblock
{
    fn print_boot_level(&self)
    {
        println!("Printing boot Structure EXT Super Block!");
        println!("Inode Count: {}", self.inode_count);
        println!("Block Count: {}", self.block_count);
        println!("SuperUser Blocks: {}", self.superuser_blocks);
        println!("Unallocated Blocks: {}", self.unallocated_blocks);
        println!("Allocated Blocks: {}", self.allocated_blocks);
        println!("Superblock Location: {}", self.superblock_loc);
        println!("Block Size (log b2): {}", self.log_2_block_size);
        println!("Fragment Size (log b2): {}", self.log_2_frag_size);
        println!("Blocks Per Group: {}", self.blocks_per_group);
        println!("Fragments Per Group: {}", self.fragments_per_group);
        println!("Inodes per Group: {}", self.inodes_per_group);
        println!("Last Mount Time: {}", self.last_mount_time);
        println!("Last Written Time: {}", self.last_written_time);
        println!("Mount Count Check: {}", self.mount_count_check);
        println!("Mount Limit Check: {}", self.mount_limit_check);
        println!("Signature: {}", self.signature);
        println!("State: {}", self.state);
        println!("State Response: {}", self.state_response);
        println!("Minor Version: {}", self.minor_version);
        println!("Last Check: {}", self.last_check);
        println!("Force Check: {}", self.force_check);
        println!("OS ID: {}", self.os_id);
        println!("Major Version: {}", self.major_version);
        println!("User ID: {}", self.user_id);
        println!("Group ID: {}", self.groupd_id);
    }

    fn verify(&self) -> bool
    {
        self.signature == 0xef53
    }
}