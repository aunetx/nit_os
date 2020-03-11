/*
Note: this code is forked and very from github.com/noahrinehart/cmos
It is licensed under the MIT license.

MIT License

Copyright (c) 2018 Noah Rinehart

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

// external crates
use cpuio::Port;

/// The standard CMOS struct.
#[derive(Debug)]
pub struct CMOS {
    address_port: Port<u8>,
    data_port: Port<u8>,
}

/// Implements the CMOS struct.
impl CMOS {
    /// Create a new CMOS struct.
    ///
    /// ## Safety
    ///
    /// This function is unsafe due to the creation of port I/O.
    ///
    /// ## Examples
    ///
    /// ```rust, no_run
    /// let mut cmos = unsafe { CMOS::new() };
    /// ```
    pub unsafe fn new() -> CMOS {
        CMOS {
            address_port: Port::<u8>::new(0x70),
            data_port: Port::<u8>::new(0x71),
        }
    }

    /// Read all the registers in CMOS.
    ///
    /// ## Examples
    ///
    /// ```rust, no_run
    /// let mut cmos = unsafe { CMOS::new() };
    /// // blank array to read into
    /// let mut cmos_values: [u8; 128] = [0; 128];
    /// // read values into provided array
    /// cmos.read_all(&mut cmos_values);
    /// ```
    pub fn read_all(&mut self, output: &mut [u8; 128]) {
        for i in 0..128 {
            self.address_port.write(i);
            output[i as usize] = self.data_port.read();
        }
    }

    /// Writes to all the registers in CMOS.
    ///
    /// ## Examples
    ///
    /// Writes all 0's, probably not a super idea to actually do this.
    ///
    /// ```rust, no_run
    /// let mut cmos = unsafe { CMOS::new() };
    /// // example values to write (don't do this!)
    /// let values: [u8; 128] = [0; 128];
    /// // writes values to all CMOS registers
    /// cmos.write_all(&values);
    /// ```
    pub fn write_all(&mut self, input: &[u8; 128]) {
        for i in 0..128 {
            self.address_port.write(i);
            self.data_port.write(input[i as usize]);
        }
    }

    /// Reads from a single register in CMOS.
    ///
    /// ## Examples
    ///
    /// ```rust, no_run
    /// let mut cmos = unsafe { CMOS::new() };
    /// // read from register 0x04 in the CMOS
    /// let reg_4 = cmos.read(0x04);
    /// ```
    pub fn read(&mut self, reg: u8) -> u8 {
        self.address_port.write(reg);
        self.data_port.read()
    }

    /// Writes to a singe register in CMOS.
    ///
    /// ## Examples
    ///
    /// Writes `0x08` to register `0x04`.
    ///
    /// ```rust, no_run
    /// let mut cmos = unsafe { CMOS::new() };
    /// // write 0x08 into register 0x04
    /// cmos.write(0x04, 0x08);
    /// ```
    pub fn write(&mut self, reg: u8, val: u8) {
        self.address_port.write(reg);
        self.data_port.write(val);
    }

    /// Reads and checks the status of the update in progress flag.
    /// When reading from the RTC, it's best to read until this flag is 0.
    ///
    /// More info found [here](https://wiki.osdev.org/CMOS#RTC_Update_In_Progress).
    ///
    /// ## Examples
    ///
    /// ```rust, no_run
    /// let mut cmos = unsafe { CMOS::new() };
    /// let mut reg0;
    /// // read register 0x00 until progress flag not 0
    /// while cmos.get_update_in_progress_flag() != 0 {
    ///     reg0 = cmos.read(0x00);
    /// }
    /// ```
    pub fn get_update_in_progress_flag(&mut self) -> u8 {
        self.read(0x0A) & 0x80
    }

    fn read_into_rtc(&mut self, rtc_time: &mut RTCDateTime) {
        while self.get_update_in_progress_flag() != 0 {
            rtc_time.second = self.read(0x00);
            rtc_time.minute = self.read(0x02);
            rtc_time.hour = self.read(0x04);
            rtc_time.day = self.read(0x07);
            rtc_time.month = self.read(0x08);
            rtc_time.year = self.read(0x09) as usize;
        }
    }

    /// Reads from the RTC part of CMOS. Unfortunatly, it is pretty slow because of the checks needed for exact accuracy.
    ///
    /// Returns an [`RTCDateTime`] struct, which includes all date time fields.\
    /// This method automatically converts BCD to binary values and 12 hours to 24 hour if necessary.
    ///
    /// ## Examples
    ///
    /// ```rust, no_run
    /// let mut cmos = unsafe { CMOS::new() };
    /// // get current RTC by current year of 2020
    /// let rtc = cmos.read_rtc(CMOSCenturyHandler::CurrentYear(2020));
    /// ```
    // FIXME this method is way too slow to be used inside the kernel
    pub fn read_rtc(&mut self, century_handler: CMOSCenturyHandler) -> RTCDateTime {
        let mut rtc_time = RTCDateTime {
            second: 0,
            minute: 0,
            hour: 0,
            day: 0,
            month: 0,
            year: 0,
        };

        let mut century = 0;
        if let CMOSCenturyHandler::CenturyRegister(century_reg) = century_handler {
            century = self.read(century_reg);
        }

        let mut last_second;
        let mut last_minute;
        let mut last_hour;
        let mut last_day;
        let mut last_month;
        let mut last_year;
        let mut last_century;

        // note: this uses the "read registers until you get the same values twice in a row" technique
        //       to avoid getting dodgy/inconsistent values due to RTC updates
        self.read_into_rtc(&mut rtc_time);

        loop {
            last_second = rtc_time.second;
            last_minute = rtc_time.minute;
            last_hour = rtc_time.hour;
            last_day = rtc_time.day;
            last_month = rtc_time.month;
            last_year = rtc_time.year;
            last_century = century;

            self.read_into_rtc(&mut rtc_time);

            if last_second != rtc_time.second
                || last_minute != rtc_time.minute
                || last_hour != rtc_time.hour
                || last_day != rtc_time.day
                || last_month != rtc_time.month
                || last_year != rtc_time.year
                || last_century != century
            {
                break;
            }
        }

        let register_b = self.read(0x0B);

        // convert BCD to binary values if necessary
        if (register_b & 0x04) == 0 {
            rtc_time.second = (rtc_time.second & 0x0F) + ((rtc_time.second / 16) * 10);
            rtc_time.minute = (rtc_time.minute & 0x0F) + ((rtc_time.minute / 16) * 10);
            rtc_time.hour = ((rtc_time.hour & 0x0F) + (((rtc_time.hour & 0x70) / 16) * 10))
                | (rtc_time.hour & 0x80);
            rtc_time.day = (rtc_time.day & 0x0F) + ((rtc_time.day / 16) * 10);
            rtc_time.month = (rtc_time.month & 0x0F) + ((rtc_time.month / 16) * 10);
            rtc_time.year = (rtc_time.year & 0x0F) + ((rtc_time.year / 16) * 10);

            if let CMOSCenturyHandler::CenturyRegister(_) = century_handler {
                century = (century & 0x0F) + ((century / 16) * 10);
            }
        }

        // convert 12 hour clock to 24 hour clock if necessary
        if ((register_b & 0x02) == 0) && ((rtc_time.hour & 0x80) != 0) {
            rtc_time.hour = ((rtc_time.hour & 0x7F) + 12) % 24;
        }

        // calculate the full (4-digit) year
        match century_handler {
            CMOSCenturyHandler::CenturyRegister(_) => rtc_time.year += (century as usize) * 100,
            CMOSCenturyHandler::CurrentYear(current_year) => {
                rtc_time.year += (current_year / 100) * 100;
                if rtc_time.year < current_year {
                    rtc_time.year += 100;
                }
            }
        }

        rtc_time
    }
}

/// Enum for determining how to calculate the year when reading the RTC.
#[derive(Debug, Clone, Copy)]
pub enum CMOSCenturyHandler {
    /// This option is for providing the number of the century register in the RTC.
    CenturyRegister(u8),
    /// This option is for providing the current year as a backup.
    CurrentYear(usize),
}

/// Results struct from reading RTC with self-explanatory fields.
#[derive(Debug, Clone, Copy)]
pub struct RTCDateTime {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: usize,
}

// ! ------------- tests -------------

// internal functions used
#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
fn test_cmos_rtc() {
    serial_print!("test_cmos_rtc... ");

    let mut cmos = unsafe { CMOS::new() };
    let _ = cmos.read_rtc(CMOSCenturyHandler::CenturyRegister(0x32));

    serial_println!("[ok]");
}
