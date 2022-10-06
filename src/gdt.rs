use crate::interrupts::TSS;
use lazy_static::lazy_static;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

// Handler for Global Descriptor Table initialization.
struct GDTLoader {
    gdt: GlobalDescriptorTable,
    selectors: Selectors,
}

lazy_static! {
    static ref GDT_LOADER: GDTLoader = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        GDTLoader {
            gdt: gdt,
            selectors: Selectors {
                code_selector,
                tss_selector,
            },
        }
    };
}

pub fn init() -> () {
    GDT_LOADER.gdt.load();
    unsafe {
        CS::set_reg(GDT_LOADER.selectors.code_selector);
        load_tss(GDT_LOADER.selectors.tss_selector);
    }
}
