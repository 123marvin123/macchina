use crate::traits::*;
use crate::windows::bindings::windows::win32::system_services::PSTR;
use winreg::enums::*;
use winreg::RegKey;
use crate::windows::bindings::windows::win32::system_services::SYSTEM_POWER_STATUS;

mod bindings {
    ::windows::include_bindings!();
}

use bindings::{
    windows::win32::system_services::GlobalMemoryStatusEx,
    windows::win32::system_services::MEMORYSTATUSEX,
    windows::win32::windows_programming::GetComputerNameExA,
    windows::win32::windows_programming::GetUserNameA,
    windows::win32::windows_programming::GetTickCount64,
    windows::win32::system_services::GetSystemPowerStatus
};
use crate::traits::ReadoutError::MetricNotAvailable;

pub struct WindowsBatteryReadout;

impl BatteryReadout for WindowsBatteryReadout {
    fn new() -> Self {
        WindowsBatteryReadout {}
    }

    fn percentage(&self) -> Result<String, ReadoutError> {
        let power_state = WindowsBatteryReadout::get_power_status()?;

        if power_state.battery_life_percent != 255 {
            return Ok(power_state.battery_life_percent.to_string());
        }

        Err(ReadoutError::MetricNotAvailable)
    }

    fn status(&self) -> Result<String, ReadoutError> {
        let power_state = WindowsBatteryReadout::get_power_status()?;

        return match power_state.ac_line_status {
            0 => Ok(String::from("FALSE")),
            1 => Ok(String::from("TRUE")),
            _ => Err(MetricNotAvailable)
        };
    }
}

impl WindowsBatteryReadout {
    fn get_power_status() -> Result<SYSTEM_POWER_STATUS, ReadoutError> {
        let mut power_state = SYSTEM_POWER_STATUS::default();

        if unsafe { GetSystemPowerStatus(&mut power_state) }.as_bool() {
            return Ok(power_state);
        }

        Err(ReadoutError::Other(String::from("Call to GetSystemPowerStatus failed.")))
    }
}

pub struct WindowsKernelReadout;

impl KernelReadout for WindowsKernelReadout {
    fn new() -> Self {
        WindowsKernelReadout {}
    }

    fn os_release(&self) -> Result<String, ReadoutError> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let current_windows_not =
            hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")?;

        let nt_build: String = current_windows_not.get_value("CurrentBuild")?;

        Ok(nt_build)
    }

    fn os_type(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Windows NT"))
    }

    fn pretty_kernel(&self) -> Result<String, ReadoutError> {
        Ok(format!("{} {}", self.os_type()?, self.os_release()?))
    }
}

pub struct WindowsMemoryReadout;

impl MemoryReadout for WindowsMemoryReadout {
    fn new() -> Self {
        WindowsMemoryReadout {}
    }

    fn total(&self) -> Result<u64, ReadoutError> {
        let memory_status = WindowsMemoryReadout::get_memory_status()?;
        Ok(memory_status.ull_total_phys / 1024u64)
    }

    fn used(&self) -> Result<u64, ReadoutError> {
        let memory_status = WindowsMemoryReadout::get_memory_status()?;
        Ok((memory_status.ull_total_phys - memory_status.ull_avail_phys) / 1024u64)
    }
}

impl WindowsMemoryReadout {
    fn get_memory_status() -> Result<MEMORYSTATUSEX, ReadoutError> {
        let mut memory_status = MEMORYSTATUSEX::default();
        memory_status.dw_length = std::mem::size_of_val(&memory_status) as u32;

        if !unsafe { GlobalMemoryStatusEx(&mut memory_status) }.as_bool() {
            return Err(ReadoutError::Other(String::from(
                "GlobalMemoryStatusEx returned a zero \
            return \
            code.",
            )));
        }

        Ok(memory_status)
    }
}

pub struct WindowsGeneralReadout;

impl GeneralReadout for WindowsGeneralReadout {
    fn new() -> Self {
        WindowsGeneralReadout {}
    }

    fn username(&self) -> Result<String, ReadoutError> {
        let mut size = 0;
        unsafe { GetUserNameA(PSTR(std::ptr::null_mut()), &mut size) };

        if size == 0 {
            return Err(ReadoutError::Other(String::from(
                "Call to \"GetUserNameA\" failed.",
            )));
        }

        let mut username = Vec::with_capacity(size as usize);
        if !unsafe { GetUserNameA(PSTR(username.as_mut_ptr()), &mut size) }.as_bool() {
            return Err(ReadoutError::Other(String::from(
                "Call to \"GetUserNameA\" failed.",
            )));
        }

        unsafe {
            username.set_len(size as usize);
        }

        let mut str = match String::from_utf8(username) {
            Ok(str) => str,
            Err(e) => {
                return Err(ReadoutError::Other(format!(
                    "String from \"GetUserNameA\" \
            was not valid UTF-8: {}",
                    e
                )))
            }
        };

        str.pop(); //remove null terminator from string.

        Ok(str)
    }

    fn hostname(&self) -> Result<String, ReadoutError> {
        use bindings::windows::win32::windows_programming::COMPUTER_NAME_FORMAT;

        let mut size = 0;
        unsafe {
            GetComputerNameExA(
                COMPUTER_NAME_FORMAT::ComputerNameDnsHostname,
                PSTR(std::ptr::null_mut()),
                &mut size,
            )
        };

        if size == 0 {
            return Err(ReadoutError::Other(String::from(
                "Call to \"GetComputerNameExA\" failed.",
            )));
        }

        let mut hostname = Vec::with_capacity(size as usize);
        if unsafe {
            GetComputerNameExA(
                COMPUTER_NAME_FORMAT::ComputerNameDnsHostname,
                PSTR(hostname.as_mut_ptr()),
                &mut size,
            )
        } == false
        {
            return Err(ReadoutError::Other(String::from(
                "Call to \"GetComputerNameExA\" failed.",
            )));
        }

        unsafe { hostname.set_len(size as usize) };

        let str = match String::from_utf8(hostname) {
            Ok(str) => str,
            Err(e) => {
                return Err(ReadoutError::Other(format!(
                    "String from \"GetComputerNameExA\" \
            was not valid UTF-8: {}",
                    e
                )))
            }
        };

        Ok(str)
    }

    fn cpu_model_name(&self) -> Result<String, ReadoutError> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let central_processor =
            hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0")?;

        let processor_name: String = central_processor.get_value("ProcessorNameString")?;

        Ok(processor_name)
    }

    fn uptime(&self) -> Result<String, ReadoutError> {
        let tick_count = unsafe { GetTickCount64() };
        let duration = std::time::Duration::from_millis(tick_count);

        Ok(duration.as_secs().to_string())
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let sys_info =
            hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\SystemInformation")?;

        let manufacturer: String = sys_info.get_value("SystemManufacturer").unwrap();
        let model: String = sys_info.get_value("SystemProductName").unwrap();

        Ok(format!("{} {}", manufacturer, model))
    }

    fn os_name(&self) -> Result<String, ReadoutError> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let nt_current =
            hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")?;

        let product_name: String = nt_current.get_value("ProductName").unwrap();
        let product_version: String = nt_current.get_value("DisplayVersion").unwrap();
        let release_id: String = nt_current.get_value("ReleaseId").unwrap();

        Ok(format!("{} {} ({})", product_name, product_version, release_id))
    }

}

pub struct WindowsProductReadout;

impl ProductReadout for WindowsProductReadout {
    fn new() -> Self {
        WindowsProductReadout {}
    }
}

pub struct WindowsPackageReadout;

impl PackageReadout for WindowsPackageReadout {
    fn new() -> Self {
        WindowsPackageReadout {}
    }
}
