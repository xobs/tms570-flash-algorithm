#![allow(clippy::all)]

/// This is used to indicate what F021 Bank Technology the bank is
#[repr(u8)]
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug)]
pub enum Fapi_FlashBankTechType {
    Fapi_FLEP = 0,
    Fapi_FLEE = 1,
    Fapi_FLES = 2,
    Fapi_FLHV = 3,
}

pub const FMSTAT_ADDRESS: *mut u32 = 0xFFF87054u32 as *mut u32;

impl Default for Fapi_FlashBankTechType {
    fn default() -> Self {
        Fapi_FlashBankTechType::Fapi_FLEP
    }
}

#[repr(C)]
#[allow(non_camel_case_types, non_snake_case, dead_code)]
#[derive(Default, Debug)]
pub struct Fapi_FlashBankSectorsType {
    pub oFlashBankTech: u8,
    pub u32NumberOfSectors: u32,
    pub u32BankStartAddress: u32,
    pub au16SectorSizes: [u16; 16],
}

/// This contains all the possible modes used in the Fapi_IssueAsyncProgrammingCommand().
#[repr(u8)]
#[allow(non_camel_case_types, dead_code)]
pub enum Fapi_FlashProgrammingCommandsType {
    /// This is the default mode for the command and will auto generate the ECC for the provided data buffer
    Fapi_AutoEccGeneration,
    /// Command will only process the data buffer
    Fapi_DataOnly,
    /// Command will only process the ecc buffer
    Fapi_EccOnly,
    /// Command will process data and ecc buffers
    Fapi_DataAndEcc,
}

//// This contains all the possible Flash State Machine commands.
#[repr(u8)]
#[allow(non_camel_case_types, dead_code)]
pub enum Fapi_FlashStateCommandsType {
    Fapi_ProgramData = 0x02,
    Fapi_EraseSector = 0x06,
    Fapi_EraseBank = 0x08,
    Fapi_ValidateSector = 0x0E,
    Fapi_ClearStatus = 0x10,
    Fapi_ProgramResume = 0x14,
    Fapi_EraseResume = 0x16,
    Fapi_ClearMore = 0x18,
}

#[repr(u8)]
#[allow(non_camel_case_types, dead_code)]
/// This is used to indicate which Flash bank is being used.
pub enum Fapi_FlashBankType {
    Fapi_FlashBank0 = 0,
    Fapi_FlashBank1 = 1,
    Fapi_FlashBank2 = 2,
    Fapi_FlashBank3 = 3,
    Fapi_FlashBank4 = 4,
    Fapi_FlashBank5 = 5,
    Fapi_FlashBank6 = 6,
    Fapi_FlashBank7 = 7,
}

impl TryFrom<u16> for Fapi_FlashBankType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Fapi_FlashBank0),
            1 => Ok(Self::Fapi_FlashBank1),
            2 => Ok(Self::Fapi_FlashBank2),
            3 => Ok(Self::Fapi_FlashBank3),
            4 => Ok(Self::Fapi_FlashBank4),
            5 => Ok(Self::Fapi_FlashBank5),
            6 => Ok(Self::Fapi_FlashBank6),
            7 => Ok(Self::Fapi_FlashBank7),
            _ => Err("Invalid flash bank"),
        }
    }
}

#[repr(u8)]
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, PartialEq)]
/// This is the master type containing all possible returned status codes
pub enum Fapi_StatusType {
    /// Function completed successfully
    Fapi_Status_Success = 0,
    /// FSM is Busy
    Fapi_Status_FsmBusy,
    /// FSM is Ready
    Fapi_Status_FsmReady,
    /// Generic Function Fail code
    Fapi_Error_Fail,
    /// One of the pointer parameters is a null pointer
    Fapi_Error_NullPointer,
    /// Command used is invalid for the function called
    Fapi_Error_InvalidCommand,
    /// Returned if the ECC Address given to a function is invalid for that function
    Fapi_Error_InvalidEccAddress,
    /// Returned if OTP checksum does not match expected value
    Fapi_Error_OtpChecksumMismatch,
    /// Returned if FClk is above max FClk value - FClk is a calculated from HClk RWAIT/EWAIT
    Fapi_Error_InvalidHclkValue,
    /// Returned if the specified bank does not exist
    Fapi_Error_InvalidBank,
    /// Returned if the specified Address does not exist in Flash or OTP
    Fapi_Error_InvalidAddress,
    /// Returned if the specified read mode does not exist
    Fapi_Error_InvalidReadMode,
    /// Returned if Data buffer size specified exceeds Data bank width
    Fapi_Error_AsyncIncorrectDataBufferLength,
    /// Returned if ECC buffer size specified exceeds ECC bank width
    Fapi_Error_AsyncIncorrectEccBufferLength,
    /// Returned if Data buffer size either is not 64bit aligned or Data length exceeds amount ECC supplied
    Fapi_Error_AsyncDataEccBufferLengthMismatch,
}

#[allow(non_camel_case_types, dead_code)]
#[repr(u8)]
#[derive(Debug)]
pub enum Fapi_ApiProductionStatusType {
    /// For internal TI use only.  Not intended to be used by customers
    Alpha_Internal,
    /// Early Engineering release.  May not be functionally complete
    Alpha,
    /// For internal TI use only.  Not intended to be used by customers
    Beta_Internal,
    /// Functionally complete, to be used for testing and validation
    Beta,
    /// Fully validated, functionally complete, ready for production use
    Production,
}

#[allow(non_camel_case_types, non_snake_case, dead_code)]
#[repr(C)]
#[derive(Debug)]
pub struct Fapi_LibraryInfoType {
    pub u8ApiMajorVersion: u8,
    pub u8ApiMinorVersion: u8,
    pub u8ApiRevision: u8,
    pub oApiProductionStatus: u8,
    pub u32ApiBuildNumber: u32,
    pub u8ApiTechnologyType: u8,
    pub u8ApiTechnologyRevision: u8,
    pub u8ApiEndianness: u8,
    pub u32ApiCompilerVersion: u32,
}

#[allow(non_camel_case_types, non_snake_case, dead_code)]
#[repr(C)]
#[derive(Debug)]
pub struct Fapi_DeviceInfoType {
    pub u16Reserved: u16,
    pub u16NumberOfBanks: u16,
    pub u16DevicePackage: u16,
    pub u16DeviceMemorySize: u16,
    pub u32AsicId: u32,
    pub u32LotNumber: u32,
    pub u16FlowCheck: u16,
    pub u16WaferNumber: u16,
    pub u16WaferXCoordinate: u16,
    pub u16WaferYCoordinate: u16,
}

#[allow(non_camel_case_types, non_snake_case, dead_code)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct Fapi_FlashStatusWordType {
    pub au32StatusWord: [u32; 4],
}

// A dummy implementation that is required when doing blank checks
#[unsafe(no_mangle)]
extern "C" fn Fapi_serviceWatchdogTimer() -> u32 {
    0
}

#[allow(dead_code)]
unsafe extern "C" {
    pub fn Fapi_initializeFlashBanks(u32HclkFrequency: u32) -> u32 /* Fapi_StatusType */;

    pub fn Fapi_setActiveFlashBank(oNewFlashBank: Fapi_FlashBankType) -> u32 /* Fapi_StatusType */;

    pub fn Fapi_enableMainBankSectors(u16SectorsEnables: u16) -> u32 /* Fapi_StatusType */;

    pub fn Fapi_enableEepromBankSectors(
        u32SectorsEnables_31_0: u32,
        u32SectorsEnables_63_32: u32,
    ) -> u32 /* Fapi_StatusType */;

    pub fn Fapi_issueAsyncCommand(
        oCommand: Fapi_FlashStateCommandsType,
    ) -> u32 /* Fapi_StatusType */;

    pub fn Fapi_issueAsyncCommandWithAddress(
        oCommand: Fapi_FlashStateCommandsType,
        pu32StartAddress: *mut u32,
    ) -> u32 /* Fapi_StatusType */;

    pub fn Fapi_issueProgrammingCommand(
        pu32StartAddress: *mut u32,
        pu8DataBuffer: *const u8,
        u8DataBufferSizeInBytes: u8,
        pu8EccBuffer: *const u8,
        u8EccBufferSizeInBytes: u8,
        oMode: Fapi_FlashProgrammingCommandsType,
    ) -> u32 /* Fapi_StatusType */;

    pub fn Fapi_getBankSectors(
        oBank: Fapi_FlashBankType,
        poFlashBankSectors: *mut Fapi_FlashBankSectorsType,
    ) -> u32 /* Fapi_StatusType */;

    pub fn Fapi_getLibraryInfo() -> Fapi_LibraryInfoType;

    pub fn Fapi_getDeviceInfo() -> Fapi_DeviceInfoType;

    pub fn Fapi_doBlankCheck(
        pu32StartAddress: *const u32,
        u32Length: u32,
        poFlashStatusWord: *mut Fapi_FlashStatusWordType,
    ) -> u32 /* Fapi_StatusType */;

    pub fn Fapi_doBlankCheckByByte(
        pu32StartAddress: *const u32,
        u32Length: u32,
        poFlashStatusWord: *mut Fapi_FlashStatusWordType,
    ) -> u32 /* Fapi_StatusType */;

    pub fn Fapi_flushPipeline();
}
