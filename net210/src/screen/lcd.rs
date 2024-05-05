use k210_soc::dmac::{dma_channel, DMAC};
use k210_soc::spi::{SPI, work_mode, frame_format, aitm, tmod};
use k210_soc::{fpioa, gpio, gpiohs, sysctl};
use k210_soc::fpioa::{function, io, pull};
use super::command::LCD_COMMAND;

use crate::tools::timer::sleep;

pub struct Lcd<'a, SPI> {
    spi : SPI,
    dmc : &'a DMAC, 
    channel : dma_channel,
    width : u16,
    height : u16
}
const DC_GPIO_NUM : u8 = 22;
const RST_GPIO_NUM : u8 = 21;
const SPI_CS : u32 = 3;
pub const LCD_X_MAX: u16 = 240; // width
pub const LCD_Y_MAX: u16 = 320; // heitght

#[derive(Clone, Copy)]
pub enum LCD_DIRECTION {
    XY_RLUD = 0x00,
    YX_RLUD = 0x20,
    XY_LRUD = 0x40,
    YX_LRUD = 0x60,
    XY_RLDU = 0x80,
    YX_RLDU = 0xA0,
    XY_LRDU = 0xC0,
    YX_LRDU = 0xE0,
}
pub const DIR_XY_MASK: u8 = 0x20;
pub const DIR_MASK: u8 = 0xE0;

impl<'a, X : SPI> Lcd<'a, X> {
    pub fn new(spi : X, dmc : &'a DMAC, channel : dma_channel) -> Self {
        fpioa::set_function(io::LCD_DC, function::gpiohs(DC_GPIO_NUM));
        fpioa::set_io_pull(io::LCD_DC, pull::DOWN);
        gpiohs::set_direction(DC_GPIO_NUM, gpio::direction::OUTPUT);
        gpiohs::set_pin(DC_GPIO_NUM, true);

        fpioa::set_function(io::LCD_RST, function::gpiohs(RST_GPIO_NUM));
        fpioa::set_io_pull(io::LCD_RST, pull::DOWN);
        gpiohs::set_direction(RST_GPIO_NUM, gpio::direction::OUTPUT);
        gpiohs::set_pin(RST_GPIO_NUM, true);

        fpioa::set_function(io::LCD_CS, fpioa::function::SPI0_SS3);
        fpioa::set_function(io::LCD_WR, fpioa::function::SPI0_SCLK);
        sysctl::set_spi0_dvp_data(true);

        spi.set_clk_rate(18_000_000);
        spi.configure(
            work_mode::MODE0,
            frame_format::OCTAL,
            8, /*data bits*/
            0, /*endian*/
            8, /*instruction length*/
            0, /*address length*/
            0, /*wait cycles*/
            aitm::AS_FRAME_FORMAT,
            tmod::TRANS,
        );
        Self{
            spi, 
            dmc, 
            channel,
            width : 0,
            height : 0
        }
    }
    pub fn on(&mut self) {
        self.rst_false();
        sleep(10);
        self.rst_true();
        sleep(10);
        self.write_command(LCD_COMMAND::SWRESET);

        self.write_command(LCD_COMMAND::COLMOD);
        self.write_byte(&[0x55]);

        self.set_direction(LCD_DIRECTION::XY_LRUD);

        /*display on*/
        self.write_command(LCD_COMMAND::SLPOUT);
        self.write_command(LCD_COMMAND::NORON);
        self.write_command(LCD_COMMAND::DISPON);
        sleep(10);
        let color = 0x41b7;
        let data = (color << 16) | color;
        self.set_area(0, 0, LCD_X_MAX - 1, LCD_Y_MAX - 1);
        self.fill_data(data, usize::from(LCD_X_MAX) * usize::from(LCD_Y_MAX));
    }
    fn dc_command(&self) {
        gpiohs::set_pin(DC_GPIO_NUM, false);
    }
    fn dc_data(&self) {
        gpiohs::set_pin(DC_GPIO_NUM, true);
    }
    fn rst_true(&self) {
        gpiohs::set_pin(RST_GPIO_NUM, true);

    }
    fn rst_false(&self) {
        gpiohs::set_pin(RST_GPIO_NUM, false);
    }

    pub fn write_command(&self, cmd: LCD_COMMAND) {
        self.dc_command();
        self.spi.configure(
            work_mode::MODE0,
            frame_format::OCTAL,
            8, /*data bits*/
            0, /*endian*/
            8, /*instruction length*/
            0, /*address length*/
            0, /*wait cycles*/
            aitm::AS_FRAME_FORMAT,
            tmod::TRANS,
        );
        self.spi.send_data_dma(self.dmc, self.channel, SPI_CS, &[cmd as u32]);
    }

    fn write_byte(&self, data_buf: &[u32]) {
        self.dc_data();
        self.spi.configure(
            work_mode::MODE0,
            frame_format::OCTAL,
            8, /*data bits*/
            0, /*endian*/
            0, /*instruction length*/
            8, /*address length*/
            0, /*wait cycles*/
            aitm::AS_FRAME_FORMAT,
            tmod::TRANS,
        );
        self.spi.send_data_dma(self.dmc, self.channel, SPI_CS, data_buf);
    }

    pub fn set_area(&self, x1: u16, y1: u16, x2: u16, y2: u16) {
        self.write_command(LCD_COMMAND::CASET);
        self.write_byte(&[
            (x1 >> 8).into(),
            (x1 & 0xff).into(),
            (x2 >> 8).into(),
            (x2 & 0xff).into(),
        ]);

        self.write_command(LCD_COMMAND::RASET);
        self.write_byte(&[
            (y1 >> 8).into(),
            (y1 & 0xff).into(),
            (y2 >> 8).into(),
            (y2 & 0xff).into(),
        ]);

        self.write_command(LCD_COMMAND::RAMWR);
    }

    pub fn fill_data(&self, data: u32, length: usize) {
        self.dc_data();
        self.spi.configure(
            work_mode::MODE0,
            frame_format::OCTAL,
            32, /*data bits*/
            0,  /*endian*/
            0,  /*instruction length*/
            32, /*address length*/
            0,  /*wait cycles*/
            aitm::AS_FRAME_FORMAT,
            tmod::TRANS,
        );
        println!("{}", length);
        self.spi.fill_data_dma(self.dmc, self.channel, SPI_CS, data, length);
        println!("OUT");
    }

    pub fn set_direction(&mut self, dir: LCD_DIRECTION) {
        if ((dir as u8) & DIR_XY_MASK) != 0 {
            self.width = LCD_Y_MAX;
            self.height = LCD_X_MAX;
        } else {
            self.width = LCD_X_MAX;
            self.height = LCD_Y_MAX;
        }

        self.write_command(LCD_COMMAND::MADCTL);
        self.write_byte(&[dir as u32]);
    }

}