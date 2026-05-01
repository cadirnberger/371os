use core::alloc;
#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();
pub const HEAP_START: usize = 0x_C371_0000; // CS-371
pub const HEAP_SIZE: usize = 1 << 16;  // Arbitrary
pub struct Dummy;

unsafe impl alloc::GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: alloc::Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: alloc::Layout) {
        panic!("dealloc should be never called")
    }
}
pub fn init_heap(
    mapper: &mut impl x86_64::structures::paging::Mapper<x86_64::structures::paging::Size4KiB>,
    frame_allocator: &mut impl x86_64::structures::paging::FrameAllocator<
        x86_64::structures::paging::Size4KiB,
    >,
) -> Option<()> {
    let page_range = {
        let heap_start = x86_64::VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = x86_64::structures::paging::Page::containing_address(heap_start);
        let heap_end_page = x86_64::structures::paging::Page::containing_address(heap_end);
        x86_64::structures::paging::Page::range_inclusive(heap_start_page, heap_end_page)
    };

    let flags = x86_64::structures::paging::PageTableFlags::PRESENT
        | x86_64::structures::paging::PageTableFlags::WRITABLE;
    for page in page_range {
        let frame = match frame_allocator.allocate_frame() {
            Some(f) => f,
            _ => return None,
        };
        unsafe {
            match mapper.map_to(page, frame, flags, frame_allocator) {
                Ok(m) => m.flush(),
                _ => return None,
            };
            ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
        }
    }

    return Some(());
}
use x86_64::structures::paging::OffsetPageTable;
use x86_64::VirtAddr;

pub unsafe fn init_mapper(phys_mem_offset: u64) -> OffsetPageTable<'static> {
    let physical_memory_offset = VirtAddr::new(phys_mem_offset);
    let level_4_table = unsafe {active_level_4_table(physical_memory_offset)
    };
    let mapper = unsafe {OffsetPageTable::new(level_4_table, physical_memory_offset)
    };
    mapper    
}
use x86_64::structures::paging::PageTable;

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    let level_4_table_ptr = (physical_memory_offset + x86_64::registers::control::Cr3::read().0.start_address().as_u64())
        .as_mut_ptr();

    let table = unsafe { &mut *level_4_table_ptr};
    table
}
