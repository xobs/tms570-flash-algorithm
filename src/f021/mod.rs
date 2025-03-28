#![allow(dead_code)]

use rtt_target::rprintln;

mod sys;

#[derive(Debug, PartialEq)]
pub enum FsmStatus {
    Ready,
    Busy,
}

/// This is used to indicate what F021 Bank Technology the bank is
#[derive(Debug, PartialEq, Default)]
pub enum FlashBankTech {
    #[default]
    FLEP = 0,
    FLEE = 1,
    FLES = 2,
    FLHV = 3,
}

#[derive(Debug, PartialEq)]
pub enum FlashBankTechError {
    InvalidTechType(u8),
}

impl core::fmt::Display for FlashBankTechError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FlashBankTechError::InvalidTechType(tech) => write!(f, "Invalid tech type: {}", tech),
        }
    }
}

impl From<FlashBankTechError> for u32 {
    fn from(val: FlashBankTechError) -> Self {
        match val {
            FlashBankTechError::InvalidTechType(e) => e.into(),
        }
    }
}

impl core::error::Error for FlashBankTechError {}

impl TryFrom<u8> for FlashBankTech {
    type Error = FlashBankTechError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FlashBankTech::FLEP),
            1 => Ok(FlashBankTech::FLEE),
            2 => Ok(FlashBankTech::FLES),
            3 => Ok(FlashBankTech::FLHV),
            _ => Err(FlashBankTechError::InvalidTechType(value)),
        }
    }
}

#[derive(Debug)]
pub struct FlashStatus {
    pub non_blank_address: u32,
    pub non_blank_data: u32,
    pub comparison_data: u32,
    pub read_mode: u32,
}

#[derive(Default)]
pub struct FlashBankSectors {
    pub flash_bank_tech: FlashBankTech,
    pub number_of_sectors: u32,
    pub bank_start_address: u32,
    pub sector_sizes: [u32; 32],
}

impl FlashBankSectors {
    pub fn sector_sizes(&self) -> &[u32] {
        &self.sector_sizes[0..(self.number_of_sectors as usize)]
    }
}

impl core::fmt::Debug for FlashBankSectors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FlashBankSectors")
            .field("flash_bank_tech", &self.flash_bank_tech)
            .field("number_of_sectors", &self.number_of_sectors)
            .field("bank_start_address", &self.bank_start_address)
            .field("sector_sizes", &self.sector_sizes())
            .finish()
    }
}

/// This contains all the possible modes used in the Fapi_IssueAsyncProgrammingCommand().
pub enum FlashProgrammingCommand {
    /// This is the default mode for the command and will auto generate the ECC for the provided data buffer
    AutoEccGeneration,
    /// Command will only process the data buffer
    DataOnly,
    /// Command will only process the ecc buffer
    EccOnly,
    /// Command will process data and ecc buffers
    DataAndEcc,
}

impl From<FlashProgrammingCommand> for sys::Fapi_FlashProgrammingCommandsType {
    fn from(val: FlashProgrammingCommand) -> Self {
        match val {
            FlashProgrammingCommand::AutoEccGeneration => {
                sys::Fapi_FlashProgrammingCommandsType::Fapi_AutoEccGeneration
            }
            FlashProgrammingCommand::DataOnly => {
                sys::Fapi_FlashProgrammingCommandsType::Fapi_DataOnly
            }
            FlashProgrammingCommand::EccOnly => {
                sys::Fapi_FlashProgrammingCommandsType::Fapi_EccOnly
            }
            FlashProgrammingCommand::DataAndEcc => {
                sys::Fapi_FlashProgrammingCommandsType::Fapi_DataAndEcc
            }
        }
    }
}

/// This contains all the possible Flash State Machine commands.
pub enum FlashStateCommand {
    ProgramData = 0x02,
    EraseSector = 0x06,
    EraseBank = 0x08,
    ValidateSector = 0x0E,
    ClearStatus = 0x10,
    ProgramResume = 0x14,
    EraseResume = 0x16,
    ClearMore = 0x18,
}

impl From<FlashStateCommand> for sys::Fapi_FlashStateCommandsType {
    fn from(val: FlashStateCommand) -> Self {
        match val {
            FlashStateCommand::ProgramData => sys::Fapi_FlashStateCommandsType::Fapi_ProgramData,
            FlashStateCommand::EraseSector => sys::Fapi_FlashStateCommandsType::Fapi_EraseSector,
            FlashStateCommand::EraseBank => sys::Fapi_FlashStateCommandsType::Fapi_EraseBank,
            FlashStateCommand::ValidateSector => {
                sys::Fapi_FlashStateCommandsType::Fapi_ValidateSector
            }
            FlashStateCommand::ClearStatus => sys::Fapi_FlashStateCommandsType::Fapi_ClearStatus,
            FlashStateCommand::ProgramResume => {
                sys::Fapi_FlashStateCommandsType::Fapi_ProgramResume
            }
            FlashStateCommand::EraseResume => sys::Fapi_FlashStateCommandsType::Fapi_EraseResume,
            FlashStateCommand::ClearMore => sys::Fapi_FlashStateCommandsType::Fapi_ClearMore,
        }
    }
}

/// This is used to indicate which Flash bank is being used.
#[derive(Clone, Copy)]
pub enum FlashBank {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
}

impl core::fmt::Debug for FlashBank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl From<FlashBank> for sys::Fapi_FlashBankType {
    fn from(val: FlashBank) -> Self {
        match val {
            FlashBank::_0 => sys::Fapi_FlashBankType::Fapi_FlashBank0,
            FlashBank::_1 => sys::Fapi_FlashBankType::Fapi_FlashBank1,
            FlashBank::_2 => sys::Fapi_FlashBankType::Fapi_FlashBank2,
            FlashBank::_3 => sys::Fapi_FlashBankType::Fapi_FlashBank3,
            FlashBank::_4 => sys::Fapi_FlashBankType::Fapi_FlashBank4,
            FlashBank::_5 => sys::Fapi_FlashBankType::Fapi_FlashBank5,
            FlashBank::_6 => sys::Fapi_FlashBankType::Fapi_FlashBank6,
            FlashBank::_7 => sys::Fapi_FlashBankType::Fapi_FlashBank7,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FlashBankError {
    InvalidFlashBank(u16),
}

impl core::fmt::Display for FlashBankError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FlashBankError::InvalidFlashBank(bank) => write!(f, "Invalid flash bank: {}", bank),
        }
    }
}

impl From<FlashBankError> for u32 {
    fn from(val: FlashBankError) -> Self {
        match val {
            FlashBankError::InvalidFlashBank(e) => e.into(),
        }
    }
}

impl core::error::Error for FlashBankError {}

impl TryFrom<u16> for FlashBank {
    type Error = FlashBankError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::_0),
            1 => Ok(Self::_1),
            2 => Ok(Self::_2),
            3 => Ok(Self::_3),
            4 => Ok(Self::_4),
            5 => Ok(Self::_5),
            6 => Ok(Self::_6),
            7 => Ok(Self::_7),
            _ => Err(FlashBankError::InvalidFlashBank(value)),
        }
    }
}

#[derive(Debug, PartialEq)]
/// This is the master type containing all possible returned status codes
pub enum Status {
    /// Function completed successfully
    Success = 0,
    /// FSM is Busy
    FsmBusy,
    /// FSM is Ready
    FsmReady,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Generic Function Fail code
    Fail,
    /// One of the pointer parameters is a null pointer
    NullPointer,
    /// Command used is invalid for the function called
    InvalidCommand,
    /// Returned if the ECC Address given to a function is invalid for that function
    InvalidEccAddress,
    /// Returned if OTP checksum does not match expected value
    OtpChecksumMismatch,
    /// Returned if FClk is above max FClk value - FClk is a calculated from HClk RWAIT/EWAIT
    InvalidHclkValue,
    /// Returned if the specified bank does not exist
    InvalidBank,
    /// Returned if the specified Address does not exist in Flash or OTP
    InvalidAddress,
    /// Returned if the specified read mode does not exist
    InvalidReadMode,
    /// Returned if Data buffer size specified exceeds Data bank width
    AsyncIncorrectDataBufferLength,
    /// Returned if ECC buffer size specified exceeds ECC bank width
    AsyncIncorrectEccBufferLength,
    /// Returned if Data buffer size either is not 64bit aligned or Data length exceeds amount ECC supplied
    AsyncDataEccBufferLengthMismatch,
    /// FMC feature is not available on this device
    FeatureNotAvailable,
    /// The flash bank could not be converted
    FlashBank(FlashBankError),
    /// The flash bank tech could not be interpreted
    FlashBankTech(FlashBankTechError),
    /// Unrecognized error
    Unrecognized(u32),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Fail => write!(f, "Generic Function Fail code"),
            Error::NullPointer => write!(f, "One of the pointer parameters is a null pointer"),
            Error::InvalidCommand => write!(f, "Command used is invalid for the function called"),
            Error::InvalidEccAddress => write!(
                f,
                "The ECC Address given to a function is invalid for this function"
            ),
            Error::OtpChecksumMismatch => write!(f, "OTP checksum does not match expected value"),
            Error::InvalidHclkValue => write!(
                f,
                "FClk is above max FClk value - FClk is a calculated from HClk RWAIT/EWAIT"
            ),
            Error::InvalidBank => write!(f, "The specified bank does not exist"),
            Error::InvalidAddress => {
                write!(f, "The specified Address does not exist in Flash or OTP")
            }
            Error::InvalidReadMode => write!(f, "The specified read mode does not exist"),
            Error::AsyncIncorrectDataBufferLength => {
                write!(f, "Data buffer size specified exceeds Data bank width")
            }
            Error::AsyncIncorrectEccBufferLength => {
                write!(f, "ECC buffer size specified exceeds ECC bank width")
            }
            Error::AsyncDataEccBufferLengthMismatch => write!(
                f,
                "Data buffer size either is not 64bit aligned or Data length exceeds amount ECC supplied"
            ),
            Error::FeatureNotAvailable => write!(f, "FMC feature is not available on this device"),
            Error::FlashBank(e) => write!(f, "Flash bank error: {}", e),
            Error::FlashBankTech(e) => write!(f, "Flash bank tech error: {}", e),
            Error::Unrecognized(err) => write!(f, "Unrecognized error: {}", err),
        }
    }
}

impl From<Error> for core::num::NonZeroU32 {
    fn from(val: Error) -> Self {
        core::num::NonZeroU32::new(match val {
            Error::Fail => 3,
            Error::NullPointer => 4,
            Error::InvalidCommand => 5,
            Error::InvalidEccAddress => 6,
            Error::OtpChecksumMismatch => 7,
            Error::InvalidHclkValue => 8,
            Error::InvalidBank => 9,
            Error::InvalidAddress => 10,
            Error::InvalidReadMode => 11,
            Error::AsyncIncorrectDataBufferLength => 12,
            Error::AsyncIncorrectEccBufferLength => 13,
            Error::AsyncDataEccBufferLengthMismatch => 14,
            Error::FeatureNotAvailable => 15,
            Error::Unrecognized(e) => 0x0100_0000u32 | (Into::<u32>::into(e)),
            Error::FlashBank(flash_bank_error) => 0x0200_0000 | Into::<u32>::into(flash_bank_error),
            Error::FlashBankTech(e) => 0x0300_0000 | Into::<u32>::into(e),
        })
        .unwrap()
    }
}

impl From<u32> for Error {
    fn from(value: u32) -> Self {
        match value {
            3 => Error::Fail,
            4 => Error::NullPointer,
            5 => Error::InvalidCommand,
            6 => Error::InvalidEccAddress,
            7 => Error::OtpChecksumMismatch,
            8 => Error::InvalidHclkValue,
            9 => Error::InvalidBank,
            10 => Error::InvalidAddress,
            11 => Error::InvalidReadMode,
            12 => Error::AsyncIncorrectDataBufferLength,
            13 => Error::AsyncIncorrectEccBufferLength,
            14 => Error::AsyncDataEccBufferLengthMismatch,
            15 => Error::FeatureNotAvailable,
            _ => Error::Unrecognized(value),
        }
    }
}

impl From<FlashBankError> for Error {
    fn from(value: FlashBankError) -> Self {
        Error::FlashBank(value)
    }
}

impl From<FlashBankTechError> for Error {
    fn from(value: FlashBankTechError) -> Self {
        Error::FlashBankTech(value)
    }
}

impl core::error::Error for Error {}

impl TryFrom<u32> for Status {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            1 => Ok(Self::FsmBusy),
            2 => Ok(Self::FsmReady),
            _ => Err(Error::from(value)),
        }
    }
}

#[derive(Debug, Default)]
pub enum ApiProductionStatus {
    /// For internal TI use only.  Not intended to be used by customers
    AlphaInternal,
    /// Early Engineering release.  May not be functionally complete
    Alpha,
    /// For internal TI use only.  Not intended to be used by customers
    BetaInternal,
    /// Functionally complete, to be used for testing and validation
    Beta,
    /// Fully validated, functionally complete, ready for production use
    Production,
    /// The production status could not be determined
    #[default]
    Unknown,
}

impl From<u8> for ApiProductionStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::AlphaInternal,
            1 => Self::Alpha,
            2 => Self::BetaInternal,
            3 => Self::Beta,
            4 => Self::Production,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Default)]
pub struct LibraryInfo {
    pub major_version: u8,
    pub minor_version: u8,
    pub revision: u8,
    pub production_status: ApiProductionStatus,
    pub build_number: u32,
    pub technology_type: u8,
    pub technology_revision: u8,
    pub endianness: u8,
    pub compiler_version: u32,
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub reserved: u16,
    pub number_of_banks: u16,
    pub device_package: u16,
    pub device_memory_size: u16,
    pub asic_id: u32,
    pub lot_number: u32,
    pub flow_check: u16,
    pub wafer_number: u16,
    pub wafer_x_coordinate: u16,
    pub wafer_y_coordinate: u16,
}

pub fn initialize_flash_banks(frequency: u32) -> Result<Status, Error> {
    unsafe { sys::Fapi_initializeFlashBanks(frequency) }.try_into()
}

pub fn set_active_flash_bank(new_flash_bank: FlashBank) -> Result<Status, Error> {
    unsafe { sys::Fapi_setActiveFlashBank(new_flash_bank.into()) }.try_into()
}
pub fn enable_main_bank_sectors(sectors_enables: u16) -> Result<Status, Error> {
    unsafe { sys::Fapi_enableMainBankSectors(sectors_enables) }.try_into()
}
pub fn enable_eeprom_bank_sectors(sectors_enables: u64) -> Result<Status, Error> {
    let upper = (sectors_enables >> 32) as u32;
    let lower = sectors_enables as u32;
    unsafe { sys::Fapi_enableEepromBankSectors(lower, upper) }.try_into()
}

pub fn issue_async_command(command: FlashStateCommand) -> Result<Status, Error> {
    unsafe { sys::Fapi_issueAsyncCommand(command.into()) }.try_into()
}

pub fn disable_ecc() {
    unsafe {
        core::arch::asm!(
            "
                mrc   p15, #0x00, r0,         c1, c0,  #0x01
                bic   r0,  r0,    #0x02000000
                mcr   p15, #0x00, r0,         c1, c0,  #0x01
            "
        );
    }
}

pub fn enable_ecc() {
    unsafe {
        core::arch::asm!(
            "
                mrc   p15, #0x00, r0,         c1, c0,  #0x01
                orr   r0,  r0,    #0x02000000
                dmb
                mcr   p15, #0x00, r0,         c1, c0,  #0x01
            "
        )
    }
}

pub fn issue_async_command_with_address(
    command: FlashStateCommand,
    start_address: *mut u32,
) -> Result<Status, Error> {
    unsafe { sys::Fapi_issueAsyncCommandWithAddress(command.into(), start_address) }.try_into()
}

pub fn issue_programming_command(
    start_address: *mut u32,
    data_buffer: &[u8],
    ecc_buffer: Option<&[u8]>,
    mode: FlashProgrammingCommand,
) -> Result<Status, Error> {
    let data_buffer_ptr = data_buffer.as_ptr();
    let data_buffer_len = data_buffer
        .len()
        .try_into()
        .or(Err(Error::AsyncIncorrectDataBufferLength))?;
    let (ecc_buffer_ptr, ecc_buffer_len) = if let Some(ecc_buffer) = ecc_buffer {
        (
            ecc_buffer.as_ptr(),
            ecc_buffer
                .len()
                .try_into()
                .or(Err(Error::AsyncIncorrectEccBufferLength))?,
        )
    } else {
        (core::ptr::null(), 0)
    };
    unsafe {
        sys::Fapi_issueProgrammingCommand(
            start_address,
            data_buffer_ptr,
            data_buffer_len,
            ecc_buffer_ptr,
            ecc_buffer_len,
            mode.into(),
        )
    }
    .try_into()
}

pub fn bank_sectors(bank: FlashBank) -> Result<FlashBankSectors, Error> {
    let mut fapi_sectors = sys::Fapi_FlashBankSectorsType::default();
    let status: Result<Status, Error> =
        unsafe { sys::Fapi_getBankSectors(bank.into(), &mut fapi_sectors as *mut _) }.try_into();
    status?;
    let mut bank_sectors = FlashBankSectors {
        flash_bank_tech: fapi_sectors.oFlashBankTech.try_into()?,
        number_of_sectors: fapi_sectors.u32NumberOfSectors,
        bank_start_address: fapi_sectors.u32BankStartAddress,
        sector_sizes: Default::default(),
    };
    for (sector_size, source) in bank_sectors
        .sector_sizes
        .iter_mut()
        .zip(fapi_sectors.au16SectorSizes.iter())
    {
        *sector_size = Into::<u32>::into(*source).saturating_mul(1024);
    }
    // For this tech, only the first size is reported. All other sizes are the same.
    if bank_sectors.flash_bank_tech == FlashBankTech::FLEE {
        let size = bank_sectors.sector_sizes[0];
        for sector_size in bank_sectors.sector_sizes.iter_mut() {
            *sector_size = size;
        }
    }
    Ok(bank_sectors)
}

pub fn library_info() -> LibraryInfo {
    let info = unsafe { sys::Fapi_getLibraryInfo() };
    LibraryInfo {
        major_version: info.u8ApiMajorVersion,
        minor_version: info.u8ApiMinorVersion,
        revision: info.u8ApiRevision,
        production_status: info.oApiProductionStatus.into(),
        build_number: info.u32ApiBuildNumber,
        technology_type: info.u8ApiTechnologyType,
        technology_revision: info.u8ApiTechnologyRevision,
        endianness: info.u8ApiEndianness,
        compiler_version: info.u32ApiCompilerVersion,
    }
}

pub fn device_info() -> DeviceInfo {
    let info = unsafe { sys::Fapi_getDeviceInfo() };
    DeviceInfo {
        reserved: info.u16Reserved,
        number_of_banks: info.u16NumberOfBanks,
        device_package: info.u16DevicePackage,
        device_memory_size: info.u16DeviceMemorySize,
        asic_id: info.u32AsicId,
        lot_number: info.u32LotNumber,
        flow_check: info.u16FlowCheck,
        wafer_number: info.u16WaferNumber,
        wafer_x_coordinate: info.u16WaferXCoordinate,
        wafer_y_coordinate: info.u16WaferYCoordinate,
    }
}

pub fn fsm_running() -> FsmStatus {
    let fmstat = unsafe { sys::FMSTAT_ADDRESS.read_volatile() };
    if fmstat & (1 << 8) == 0 {
        FsmStatus::Ready
    } else {
        FsmStatus::Busy
    }
}


/// Invalidate D$ and I$
pub fn invalidate_caches() {
    unsafe {
        core::arch::asm!("
            mov   r0,#0
            dsb
            mcr   p15, #0, r0, c15, c5, #0 // dcache
            mcr   p15, #0, r0, c7, c5, #0 // icache
            dsb
        ", out("r0") _);
    }
}
pub fn blank_check(address: u32, size: u32) -> Result<(), FlashStatus> {
    let mut flash_status = sys::Fapi_FlashStatusWordType::default();
    invalidate_caches();
    // Note: `size` is in units of 32-bits.
    let status: Result<Status, Error> = unsafe {
        sys::Fapi_doBlankCheck(address as *const u32, size / 4, &mut flash_status as *mut _)
    }
    .try_into();
    if status.is_err() {
        rprintln!("Error performing blank check: {:?}", status);
        Err(FlashStatus {
            non_blank_address: flash_status.au32StatusWord[0],
            non_blank_data: flash_status.au32StatusWord[1],
            comparison_data: flash_status.au32StatusWord[2],
            read_mode: flash_status.au32StatusWord[3],
        })
    } else {
        Ok(())
    }
}

pub fn blank_check_bytewise(address: u32, size: u32) -> Result<(), FlashStatus> {
    let mut flash_status = sys::Fapi_FlashStatusWordType::default();
    // Note: The datasheet says `size` is in units of 32-bits, but it appears as though
    // it's actually in units of 8-bits.
    let status: Result<Status, Error> = unsafe {
        sys::Fapi_doBlankCheckByByte(address as *const u32, size, &mut flash_status as *mut _)
    }
    .try_into();
    if status.is_err() {
        Err(FlashStatus {
            non_blank_address: flash_status.au32StatusWord[0],
            non_blank_data: flash_status.au32StatusWord[1],
            comparison_data: flash_status.au32StatusWord[2],
            read_mode: flash_status.au32StatusWord[3],
        })
    } else {
        Ok(())
    }
}

pub fn flush() {
    unsafe { sys::Fapi_flushPipeline() }
}
