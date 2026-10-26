use core::sync::atomic::{AtomicBool, Ordering};

const UART0: usize = 0x1000_0000;
const RHR: usize = 0;
const THR: usize = 0;
const IER: usize = 1;
const FCR: usize = 2;
const LCR: usize = 3;
const LSR: usize = 5;

const LCR_BAUD_LATCH: u8 = 1 << 7;
const LCR_EIGHT_BITS: u8 = 3;
const FCR_FIFO_ENABLE: u8 = 1;
const FCR_FIFO_CLEAR: u8 = 3 << 1;
const IER_RX_ENABLE: u8 = 1 << 0;
const LSR_RX_READY: u8 = 1 << 0;
const LSR_TX_IDLE: u8 = 1 << 5;

static UART_TEST_REPORTED: AtomicBool = AtomicBool::new(false);

#[inline]
unsafe fn write_register(offset: usize, value: u8) {
    (UART0 as *mut u8).add(offset).write_volatile(value);
}

#[inline]
unsafe fn read_register(offset: usize) -> u8 {
    (UART0 as *const u8).add(offset).read_volatile()
}

pub fn init() {
    unsafe {
        write_register(IER, 0);
        write_register(LCR, LCR_BAUD_LATCH);
        write_register(0, 0x03);
        write_register(1, 0);
        write_register(LCR, LCR_EIGHT_BITS);
        write_register(FCR, FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);
    }
}

/// 在 trap 入口和 PLIC 都就绪后，才允许 UART 的接收中断进入。
pub fn enable_receive_interrupt() {
    unsafe {
        write_register(IER, IER_RX_ENABLE);
    }
}

/// 处理一次 UART 外部中断：读空 RX FIFO，并回显每个字符。
pub fn handle_interrupt() {
    while unsafe { read_register(LSR) } & LSR_RX_READY != 0 {
        let byte = unsafe { read_register(RHR) };
        match byte {
            8 | 127 => {
                putchar(8);
                putchar(b' ');
                putchar(8);
            }
            b'\r' | b'\n' => putchar(b'\n'),
            byte => putchar(byte),
        }

        // 验收标志只打印一次；直接写 UART，避免在中断上下文里争用打印锁。
        if byte == b'K' && !UART_TEST_REPORTED.swap(true, Ordering::AcqRel) {
            for character in b"lab-3 uart test: PASS byte=K\n" {
                putchar(*character);
            }
        }
    }
}

pub fn putchar(character: u8) {
    unsafe {
        while read_register(LSR) & LSR_TX_IDLE == 0 {
            core::hint::spin_loop();
        }
        write_register(THR, character);
    }
}
