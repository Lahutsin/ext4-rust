use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::mem::size_of;

// Size of superblock in bytes
const SUPERBLOCK_SIZE: usize = 1024;

// Offset of superblock in file
const SUPERBLOCK_OFFSET: u64 = 1024;

// Struct of superblock EXT4
#[repr(C)]
#[derive(Debug)]
struct Superblock {
    s_inodes_count: u32,
    s_blocks_count: u32,
    s_r_blocks_count: u32,
    s_free_blocks_count: u32,
    s_free_inodes_count: u32,
    s_first_data_block: u32,
    s_log_block_size: u32,
    s_log_cluster_size: u32,
    s_blocks_per_group: u32,
    s_clusters_per_group: u32,
    s_inodes_per_group: u32,
    s_mtime: u32,
    s_wtime: u32,
    // to do other fileds
}

impl Superblock {
    fn new() -> Self {
        Superblock {
            s_inodes_count: 0,
            s_blocks_count: 0,
            s_r_blocks_count: 0,
            s_free_blocks_count: 0,
            s_free_inodes_count: 0,
            s_first_data_block: 0,
            s_log_block_size: 0,
            s_log_cluster_size: 0,
            s_blocks_per_group: 0,
            s_clusters_per_group: 0,
            s_inodes_per_group: 0,
            s_mtime: 0,
            s_wtime: 0,
        }
    }

    fn read_from_disk(file: &mut File) -> io::Result<Self> {
        let mut superblock = Superblock::new();
        file.seek(SeekFrom::Start(SUPERBLOCK_OFFSET))?;
        let superblock_size = size_of::<Superblock>();
        let superblock_slice = unsafe {
            std::slice::from_raw_parts_mut(&mut superblock as *mut _ as *mut u8, superblock_size)
        };
        file.read_exact(superblock_slice)?;
        Ok(superblock)
    }
}

// Struct of group's descriptor
#[repr(C)]
#[derive(Debug)]
struct GroupDescriptor {
    bg_block_bitmap: u32,
    bg_inode_bitmap: u32,
    bg_inode_table: u32,
    bg_free_blocks_count: u16,
    bg_free_inodes_count: u16,
    bg_used_dirs_count: u16,
    bg_pad: u16,
    bg_reserved: [u32; 3],
}

impl GroupDescriptor {
    fn new() -> Self {
        GroupDescriptor {
            bg_block_bitmap: 0,
            bg_inode_bitmap: 0,
            bg_inode_table: 0,
            bg_free_blocks_count: 0,
            bg_free_inodes_count: 0,
            bg_used_dirs_count: 0,
            bg_pad: 0,
            bg_reserved: [0; 3],
        }
    }

    fn read_from_disk(file: &mut File, offset: u64) -> io::Result<Self> {
        let mut group_descriptor = GroupDescriptor::new();
        file.seek(SeekFrom::Start(offset))?;
        let group_descriptor_size = size_of::<GroupDescriptor>();
        let group_descriptor_slice = unsafe {
            std::slice::from_raw_parts_mut(&mut group_descriptor as *mut _ as *mut u8, group_descriptor_size)
        };
        file.read_exact(group_descriptor_slice)?;
        Ok(group_descriptor)
    }
}

/*
fn main() -> io::Result<()> {
    let mut file = File::open("/dev/sdX1")?; // any path: /dev/sdX1
    let superblock = Superblock::read_from_disk(&mut file)?;
    println!("{:?}", superblock);

    let block_size = 1024 << superblock.s_log_block_size;
    let group_descriptor_table_offset = SUPERBLOCK_OFFSET + block_size as u64;

    let group_descriptor = GroupDescriptor::read_from_disk(&mut file, group_descriptor_table_offset)?;
    println!("{:?}", group_descriptor);
    
    Ok(())
}
