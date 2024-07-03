#![no_std]
#![no_main]
#![feature(asm)]
#![feature(once_cell)]

mod hex_data;

#[cfg(feature = "defmt")]
use defmt_rtt as _; // global logger
// use panic_probe as _;

use hex_data::DATA;
use shakti_riscv_hal::gpio::{GPIOInner, GPIO_OFFSET};
use shakti_riscv_hal::uart::{UartInner, UART_OFFSET};
use shakti_riscv_hal::spi::{SPIInner, SPI_OFFSET};
use riscv::asm;
use riscv_rt::{entry};


struct Spi{
    spi: SPIInner,
}

struct UartInit{
    uart: UartInner,
}

#[entry]
fn main() -> ! {
    let mut read_addr = 0x00b0_0000;
    let mut d_count: u32 = 0;

    let mut spi = unsafe { SPIInner::new(SPI_OFFSET) };
    let mut uart = unsafe { UartInner::new(UART_OFFSET) };
    spi.init();
    
    // wait for 200 --> need to create
    uart.write_uart_string("Erasing...\n ");

    spi.flash_write_enable();

    spi.flash_erase(read_addr);

    spi.flash_status_register_read();

    uart.write_uart_string("Erase Complete.\n");

    spi.flash_write(read_addr, DATA[0] as u32);

    read_addr += 2;

    let mut i =1;
    while(d_count < DATA.len() as u32){
        // wait for200 ms
        spi.flash_write(read_addr, DATA[i] as u32);
        i += 1;
        d_count += 1;
        read_addr += 2;

        if i % 512 == 0{
            uart.write_uart_string(".");
        }

        uart.write_uart_string("\nWriting complete\n");
        uart.write_uart_string("Please reset \n");
    }
    
    loop{}
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { riscv::asm::nop() };
    }
}
