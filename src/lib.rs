#![allow(non_upper_case_globals)]

pub type LLVMBool = libc::c_int;

pub enum LLVMOpaqueContext {}
pub type LLVMContextRef = *mut LLVMOpaqueContext;

pub enum LLVMOpaqueMemoryBuffer {}
pub type LLVMMemoryBufferRef = *mut LLVMOpaqueMemoryBuffer;

pub enum LLVMOpaqueModule {}
pub type LLVMModuleRef = *mut LLVMOpaqueModule;

pub enum LLVMTarget {}
pub type LLVMTargetRef = *mut LLVMTarget;

pub enum LLVMOpaqueTargetMachine {}
pub type LLVMTargetMachineRef = *mut LLVMOpaqueTargetMachine;

#[repr(C)]
pub struct LLVMCodeGenOptLevel(pub libc::c_int);
pub const LLVMCodeGenLevelNone: LLVMCodeGenOptLevel = LLVMCodeGenOptLevel(0);
pub const LLVMCodeGenLevelLess: LLVMCodeGenOptLevel = LLVMCodeGenOptLevel(1);
pub const LLVMCodeGenLevelDefault: LLVMCodeGenOptLevel = LLVMCodeGenOptLevel(2);
pub const LLVMCodeGenLevelAggressive: LLVMCodeGenOptLevel = LLVMCodeGenOptLevel(3);

#[repr(C)]
pub struct LLVMRelocMode(pub libc::c_int);
pub const LLVMRelocDefault: LLVMRelocMode = LLVMRelocMode(0);
pub const LLVMRelocStatic: LLVMRelocMode = LLVMRelocMode(1);
pub const LLVMRelocPIC: LLVMRelocMode = LLVMRelocMode(2);
pub const LLVMRelocDynamicNoPic: LLVMRelocMode = LLVMRelocMode(3);
pub const LLVMRelocROPI: LLVMRelocMode = LLVMRelocMode(4);
pub const LLVMRelocRWPI: LLVMRelocMode = LLVMRelocMode(5);
pub const LLVMRelocROPI_RWPI: LLVMRelocMode = LLVMRelocMode(6);

#[repr(C)]
pub struct LLVMCodeModel(pub libc::c_int);
pub const LLVMCodeModelDefault: LLVMCodeModel = LLVMCodeModel(0);
pub const LLVMCodeModelJITDefault: LLVMCodeModel = LLVMCodeModel(1);
pub const LLVMCodeModelTiny: LLVMCodeModel = LLVMCodeModel(2);
pub const LLVMCodeModelSmall: LLVMCodeModel = LLVMCodeModel(3);
pub const LLVMCodeModelKernel: LLVMCodeModel = LLVMCodeModel(4);
pub const LLVMCodeModelMedium: LLVMCodeModel = LLVMCodeModel(5);
pub const LLVMCodeModelLarge: LLVMCodeModel = LLVMCodeModel(6);

#[repr(C)]
pub struct LLVMCodeGenFileType(pub libc::c_int);
pub const LLVMAssemblyFile: LLVMCodeGenFileType = LLVMCodeGenFileType(0);
pub const LLVMObjectFile: LLVMCodeGenFileType = LLVMCodeGenFileType(1);

extern "C" {
    pub fn LLVMContextCreate() -> LLVMContextRef;
    pub fn LLVMContextDispose(ContextRef: LLVMContextRef);

    pub fn LLVMCreateMemoryBufferWithContentsOfFile(
        Path: *const libc::c_char,
        OutMemBuf: &mut LLVMMemoryBufferRef,
        OutMessage: &mut *mut libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMDisposeMemoryBuffer(MemBuf: LLVMMemoryBufferRef);

    pub fn LLVMParseIRInContext(
        ContextRef: LLVMContextRef,
        MemBuf: LLVMMemoryBufferRef,
        OutM: &mut LLVMModuleRef,
        OutMessage: &mut *mut libc::c_char,
    ) -> LLVMBool;

    pub fn LLVMGetTargetFromTriple(
        Triple: *const libc::c_char,
        T: &mut LLVMTargetRef,
        ErrorMessage: &mut *mut libc::c_char,
    ) -> LLVMBool;

    pub fn LLVMCreateTargetMachine(
        T: LLVMTargetRef,
        Triple: *const libc::c_char,
        CPU: *const libc::c_char,
        Features: *const libc::c_char,
        Level: LLVMCodeGenOptLevel,
        Reloc: LLVMRelocMode,
        CodeModel: LLVMCodeModel,
    ) -> LLVMTargetMachineRef;
    pub fn LLVMDisposeTargetMachine(T: LLVMTargetMachineRef);

    pub fn LLVMTargetMachineEmitToFile(
        T: LLVMTargetMachineRef,
        M: LLVMModuleRef,
        Filename: *mut libc::c_char,
        codegen: LLVMCodeGenFileType,
        ErrorMessage: &mut *mut libc::c_char,
    ) -> LLVMBool;

    pub fn LLVMDisposeMessage(Message: *mut libc::c_char);
    pub fn LLVMDisposeModule(M: LLVMModuleRef);

    pub fn LLVMShutdown();

    pub fn LLVMInitializeX86TargetInfo();
    pub fn LLVMInitializeX86Target();
    pub fn LLVMInitializeX86TargetMC();
    pub fn LLVMInitializeX86AsmPrinter();
    pub fn LLVMInitializeX86AsmParser();

    pub fn LLVMInitializeARMTargetInfo();
    pub fn LLVMInitializeARMTarget();
    pub fn LLVMInitializeARMTargetMC();
    pub fn LLVMInitializeARMAsmPrinter();
    pub fn LLVMInitializeARMAsmParser();

    pub fn LLVMInitializeAArch64TargetInfo();
    pub fn LLVMInitializeAArch64Target();
    pub fn LLVMInitializeAArch64TargetMC();
    pub fn LLVMInitializeAArch64AsmPrinter();
    pub fn LLVMInitializeAArch64AsmParser();

    pub fn LLVMInitializeAMDGPUTargetInfo();
    pub fn LLVMInitializeAMDGPUTarget();
    pub fn LLVMInitializeAMDGPUTargetMC();
    pub fn LLVMInitializeAMDGPUAsmPrinter();
    pub fn LLVMInitializeAMDGPUAsmParser();

    pub fn LLVMInitializeMipsTargetInfo();
    pub fn LLVMInitializeMipsTarget();
    pub fn LLVMInitializeMipsTargetMC();
    pub fn LLVMInitializeMipsAsmPrinter();
    pub fn LLVMInitializeMipsAsmParser();

    pub fn LLVMInitializePowerPCTargetInfo();
    pub fn LLVMInitializePowerPCTarget();
    pub fn LLVMInitializePowerPCTargetMC();
    pub fn LLVMInitializePowerPCAsmPrinter();
    pub fn LLVMInitializePowerPCAsmParser();

    pub fn LLVMInitializeSystemZTargetInfo();
    pub fn LLVMInitializeSystemZTarget();
    pub fn LLVMInitializeSystemZTargetMC();
    pub fn LLVMInitializeSystemZAsmPrinter();
    pub fn LLVMInitializeSystemZAsmParser();

    pub fn LLVMInitializeJSBackendTargetInfo();
    pub fn LLVMInitializeJSBackendTarget();
    pub fn LLVMInitializeJSBackendTargetMC();

    pub fn LLVMInitializeSparcTargetInfo();
    pub fn LLVMInitializeSparcTarget();
    pub fn LLVMInitializeSparcTargetMC();
    pub fn LLVMInitializeSparcAsmPrinter();
    pub fn LLVMInitializeSparcAsmParser();

    pub fn LLVMInitializeNVPTXTargetInfo();
    pub fn LLVMInitializeNVPTXTarget();
    pub fn LLVMInitializeNVPTXTargetMC();
    pub fn LLVMInitializeNVPTXAsmPrinter();

    pub fn LLVMInitializeHexagonTargetInfo();
    pub fn LLVMInitializeHexagonTarget();
    pub fn LLVMInitializeHexagonTargetMC();
    pub fn LLVMInitializeHexagonAsmPrinter();
    pub fn LLVMInitializeHexagonAsmParser();

    pub fn LLVMInitializeWebAssemblyTargetInfo();
    pub fn LLVMInitializeWebAssemblyTarget();
    pub fn LLVMInitializeWebAssemblyTargetMC();
    pub fn LLVMInitializeWebAssemblyAsmPrinter();
}
