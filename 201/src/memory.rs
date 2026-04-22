use x86_64::VirtAddr;
use x86_64::PhysAddr;
use x86_64::structures::paging::PhysFrame;
use bootloader::bootinfo::MemoryRegionType;
pub unsafe fn active_level_4_table(offset: x86_64::VirtAddr) -> &'static mut x86_64::structures::paging::PageTable
{
    unsafe {
        let (frame, _) = x86_64::registers::control::Cr3::read();
    
        let phys = frame.start_address();
        let virt = offset + phys.as_u64();
        let ptr: *mut x86_64::structures::paging::PageTable = virt.as_mut_ptr();
    
        return &mut *ptr;
    }
}

pub unsafe fn translate_addr(addr: VirtAddr, offset: VirtAddr)
    -> Option<PhysAddr>
{
    unsafe {
    let (l4, _) = x86_64::registers::control::Cr3::read();
    let mut frame = l4;
    let indices = [addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()];
    for &index in &indices {
        let virt = offset + frame.start_address().as_u64();
        let ptr: *const x86_64::structures::paging::PageTable = virt.as_ptr();
        frame = match &(&*ptr)[index].frame() {
            Ok(x) => *x,
            Err(_) => return None,
        };
    }
    return Some(frame.start_address() + u64::from(addr.page_offset())); 
    }
}
pub unsafe fn init(
    offset: x86_64::VirtAddr,
) -> x86_64::structures::paging::OffsetPageTable<'static> {
    unsafe {
        let l4 = active_level_4_table(offset);
        x86_64::structures::paging::OffsetPageTable::new(l4, offset)
    }
}
pub fn create_example_mapping(
    page: x86_64::structures::paging::Page,
    mapper: &mut x86_64::structures::paging::OffsetPageTable,
    frame_allocator: &mut impl x86_64::structures::paging::FrameAllocator<
        x86_64::structures::paging::Size4KiB,
    >,
) {
    let frame =
        x86_64::structures::paging::PhysFrame::containing_address(x86_64::PhysAddr::new(0xb8000));
    let flags = x86_64::structures::paging::PageTableFlags::PRESENT
        | x86_64::structures::paging::PageTableFlags::WRITABLE;

    let map_to_result = unsafe {
        use x86_64::structures::paging::Mapper;
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}
pub struct EmptyFrameAllocator;

unsafe impl x86_64::structures::paging::FrameAllocator<x86_64::structures::paging::Size4KiB>
    for BootInfoFrameAllocator
{
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
pub struct BootInfoFrameAllocator {
    memory_map: &'static bootloader::bootinfo::MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static bootloader::bootinfo::MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // 1. iterate over memory regions
        let regions = self.memory_map.iter();

        // 2. filter usable regions
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);

        // 3. convert to address ranges
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());

        // 4. step through each frame (4KiB)
        let frame_addresses = addr_ranges
            .flat_map(|r| r.step_by(4096));

        // 5. convert to PhysFrame
        frame_addresses.map(|addr| {
            PhysFrame::containing_address(PhysAddr::new(addr))
        })
    }
}
