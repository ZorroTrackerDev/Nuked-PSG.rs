use nuked_psg_sys::*;

pub struct Chip {
    ptr: Box<ympsg_t>
}

impl Chip {
    pub fn new() -> Self {
        let mut chip = unsafe {
            Self {
                ptr: Box::new(std::mem::zeroed::<ympsg_t>())
            }
        };

        chip.init();
        chip        
    }

    pub fn init(&mut self) {
        unsafe {
            YMPSG_Init(&mut *self.ptr);
        }
    }

    pub fn write(&mut self, data: u8) {
        unsafe {
            YMPSG_Write(&mut *self.ptr, data);
        }
    }

    pub fn read(&mut self) -> u16 {
        unsafe {
            YMPSG_Read(&mut *self.ptr)
        }
    }

    pub fn set_ic(&mut self, ic: u32) {
        unsafe {
            YMPSG_SetIC(&mut *self.ptr, ic);
        }
    }

    pub fn clock(&mut self) {
        unsafe {
            YMPSG_Clock(&mut *self.ptr);
        }
    }

    pub fn get_output(&mut self) -> f32 {
        unsafe {
            YMPSG_GetOutput(&mut *self.ptr)
        }
    }

    pub fn test(&mut self, test: u16) {
        unsafe {
            YMPSG_Test(&mut *self.ptr, test);
        }
    }

    pub fn generate(&mut self) -> i32 {
        let mut output: i32 = 0;

        unsafe {
            YMPSG_Generate(&mut *self.ptr, &mut output as *mut i32);
        }

        output
    }

    pub fn write_buffered(&mut self, data: u8) {
        unsafe {
            YMPSG_WriteBuffered(&mut *self.ptr, data);
        }
    }
}

