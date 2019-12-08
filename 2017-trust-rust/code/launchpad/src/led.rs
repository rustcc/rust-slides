
use volatile_register::RW;

const RCGCGPIO: *const RW<u32> = (0x400FE000 + 0x608) as *const RW<u32>;

const GPIOF_DEN: u32 = (0x40025000 + 0x51C);
const GPIOF_DIR: u32 = (0x40025000 + 0x400); 
const GPIOF_DATA: u32 = (0x40025000 + 0x3FC);

const PORT_F: u32 = 5;
const PIN_RED: u32 = 1;
const PIN_BLUE: u32 = 2;
const PIN_GREEN: u32 = 3;

/// On-board RGB LED's on PORTF
pub struct Led {
    pin: u32,
    enable_reg: *const RW<u32>,
    dir_reg: *const RW<u32>,
    data_reg: *const RW<u32>
}

pub fn portf_init() {
    unsafe {
        (*RCGCGPIO).modify(|val| val | (1 << PORT_F));
        (*RCGCGPIO).read(); // wait for clk to start
    }
}
    
impl Led {
    fn new(pin: u32, enable: u32, dir: u32, data: u32) -> Self {
        unsafe {
            // enable the pin
            (*(enable as *const RW<u32>)).modify(|val| val | (1 << pin));
        
            // configure as output
            (*(dir as *const RW<u32>)).modify(|val| val | (1 << pin));
        
        } 
        Led { pin: pin,
              enable_reg: enable as *const RW<u32>,
              dir_reg: dir as *const RW<u32>,
              data_reg: data as *const RW<u32>,
        }
    }

    pub fn on(&self) {
        unsafe {
            (*(self.data_reg)).modify(|val| val | (1 << self.pin));
        }   
    }

    pub fn off(&self) {
        unsafe {
            (*(self.data_reg)).modify(|val| val & !(1 << self.pin));
        }
    }
}


pub fn red_led() -> Led {
    Led::new(PIN_RED, GPIOF_DEN,
             GPIOF_DIR, GPIOF_DATA)
}

pub fn green_led() -> Led {
    Led::new(PIN_GREEN, GPIOF_DEN,
             GPIOF_DIR, GPIOF_DATA)
}

pub fn blue_led() -> Led {
    Led::new(PIN_BLUE, GPIOF_DEN,
             GPIOF_DIR, GPIOF_DATA)
}

#[inline(never)]
pub fn delay(mut x: u32) {
    while x != 0 {
        x -= 1;
    }
}
