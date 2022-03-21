use std::io::{Error, SeekFrom};
use std::ptr;

use anyhow::Result;
use drop_struct_macro_derive::DropStructMacro;
use ffi_toolkit::{code_and_message_impl, free_c_str, CodeAndMessage, FCPResponseStatus};
use filecoin_proofs_api::{
    seal::SealCommitPhase2Output, PieceInfo, RegisteredAggregationProof, RegisteredPoStProof,
    RegisteredSealProof, RegisteredUpdateProof, UnpaddedBytesAmount,
};

use crate::util::types::{
    clone_as_vec_from_parts, clone_box_parts, drop_box_from_parts, fil_Array, fil_Bytes, fil_Result,
};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct fil_32ByteArray {
    pub inner: [u8; 32],
}

/// FileDescriptorRef does not drop its file descriptor when it is dropped. Its
/// owner must manage the lifecycle of the file descriptor.
pub struct FileDescriptorRef(std::mem::ManuallyDrop<std::fs::File>);

impl FileDescriptorRef {
    #[cfg(not(target_os = "windows"))]
    pub unsafe fn new(raw: std::os::unix::io::RawFd) -> Self {
        use std::os::unix::io::FromRawFd;
        FileDescriptorRef(std::mem::ManuallyDrop::new(std::fs::File::from_raw_fd(raw)))
    }
}

impl std::io::Read for FileDescriptorRef {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl std::io::Write for FileDescriptorRef {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.0.flush()
    }
}

impl std::io::Seek for FileDescriptorRef {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        self.0.seek(pos)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum fil_RegisteredSealProof {
    StackedDrg2KiBV1,
    StackedDrg8MiBV1,
    StackedDrg512MiBV1,
    StackedDrg32GiBV1,
    StackedDrg64GiBV1,
    StackedDrg2KiBV1_1,
    StackedDrg8MiBV1_1,
    StackedDrg512MiBV1_1,
    StackedDrg32GiBV1_1,
    StackedDrg64GiBV1_1,
}

impl From<RegisteredSealProof> for fil_RegisteredSealProof {
    fn from(other: RegisteredSealProof) -> Self {
        match other {
            RegisteredSealProof::StackedDrg2KiBV1 => fil_RegisteredSealProof::StackedDrg2KiBV1,
            RegisteredSealProof::StackedDrg8MiBV1 => fil_RegisteredSealProof::StackedDrg8MiBV1,
            RegisteredSealProof::StackedDrg512MiBV1 => fil_RegisteredSealProof::StackedDrg512MiBV1,
            RegisteredSealProof::StackedDrg32GiBV1 => fil_RegisteredSealProof::StackedDrg32GiBV1,
            RegisteredSealProof::StackedDrg64GiBV1 => fil_RegisteredSealProof::StackedDrg64GiBV1,

            RegisteredSealProof::StackedDrg2KiBV1_1 => fil_RegisteredSealProof::StackedDrg2KiBV1_1,
            RegisteredSealProof::StackedDrg8MiBV1_1 => fil_RegisteredSealProof::StackedDrg8MiBV1_1,
            RegisteredSealProof::StackedDrg512MiBV1_1 => {
                fil_RegisteredSealProof::StackedDrg512MiBV1_1
            }
            RegisteredSealProof::StackedDrg32GiBV1_1 => {
                fil_RegisteredSealProof::StackedDrg32GiBV1_1
            }
            RegisteredSealProof::StackedDrg64GiBV1_1 => {
                fil_RegisteredSealProof::StackedDrg64GiBV1_1
            }
        }
    }
}

impl From<fil_RegisteredSealProof> for RegisteredSealProof {
    fn from(other: fil_RegisteredSealProof) -> Self {
        match other {
            fil_RegisteredSealProof::StackedDrg2KiBV1 => RegisteredSealProof::StackedDrg2KiBV1,
            fil_RegisteredSealProof::StackedDrg8MiBV1 => RegisteredSealProof::StackedDrg8MiBV1,
            fil_RegisteredSealProof::StackedDrg512MiBV1 => RegisteredSealProof::StackedDrg512MiBV1,
            fil_RegisteredSealProof::StackedDrg32GiBV1 => RegisteredSealProof::StackedDrg32GiBV1,
            fil_RegisteredSealProof::StackedDrg64GiBV1 => RegisteredSealProof::StackedDrg64GiBV1,

            fil_RegisteredSealProof::StackedDrg2KiBV1_1 => RegisteredSealProof::StackedDrg2KiBV1_1,
            fil_RegisteredSealProof::StackedDrg8MiBV1_1 => RegisteredSealProof::StackedDrg8MiBV1_1,
            fil_RegisteredSealProof::StackedDrg512MiBV1_1 => {
                RegisteredSealProof::StackedDrg512MiBV1_1
            }
            fil_RegisteredSealProof::StackedDrg32GiBV1_1 => {
                RegisteredSealProof::StackedDrg32GiBV1_1
            }
            fil_RegisteredSealProof::StackedDrg64GiBV1_1 => {
                RegisteredSealProof::StackedDrg64GiBV1_1
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum fil_RegisteredPoStProof {
    StackedDrgWinning2KiBV1,
    StackedDrgWinning8MiBV1,
    StackedDrgWinning512MiBV1,
    StackedDrgWinning32GiBV1,
    StackedDrgWinning64GiBV1,
    StackedDrgWindow2KiBV1,
    StackedDrgWindow8MiBV1,
    StackedDrgWindow512MiBV1,
    StackedDrgWindow32GiBV1,
    StackedDrgWindow64GiBV1,
}

impl From<RegisteredPoStProof> for fil_RegisteredPoStProof {
    fn from(other: RegisteredPoStProof) -> Self {
        use RegisteredPoStProof::*;

        match other {
            StackedDrgWinning2KiBV1 => fil_RegisteredPoStProof::StackedDrgWinning2KiBV1,
            StackedDrgWinning8MiBV1 => fil_RegisteredPoStProof::StackedDrgWinning8MiBV1,
            StackedDrgWinning512MiBV1 => fil_RegisteredPoStProof::StackedDrgWinning512MiBV1,
            StackedDrgWinning32GiBV1 => fil_RegisteredPoStProof::StackedDrgWinning32GiBV1,
            StackedDrgWinning64GiBV1 => fil_RegisteredPoStProof::StackedDrgWinning64GiBV1,
            StackedDrgWindow2KiBV1 => fil_RegisteredPoStProof::StackedDrgWindow2KiBV1,
            StackedDrgWindow8MiBV1 => fil_RegisteredPoStProof::StackedDrgWindow8MiBV1,
            StackedDrgWindow512MiBV1 => fil_RegisteredPoStProof::StackedDrgWindow512MiBV1,
            StackedDrgWindow32GiBV1 => fil_RegisteredPoStProof::StackedDrgWindow32GiBV1,
            StackedDrgWindow64GiBV1 => fil_RegisteredPoStProof::StackedDrgWindow64GiBV1,
        }
    }
}

impl From<fil_RegisteredPoStProof> for RegisteredPoStProof {
    fn from(other: fil_RegisteredPoStProof) -> Self {
        use RegisteredPoStProof::*;

        match other {
            fil_RegisteredPoStProof::StackedDrgWinning2KiBV1 => StackedDrgWinning2KiBV1,
            fil_RegisteredPoStProof::StackedDrgWinning8MiBV1 => StackedDrgWinning8MiBV1,
            fil_RegisteredPoStProof::StackedDrgWinning512MiBV1 => StackedDrgWinning512MiBV1,
            fil_RegisteredPoStProof::StackedDrgWinning32GiBV1 => StackedDrgWinning32GiBV1,
            fil_RegisteredPoStProof::StackedDrgWinning64GiBV1 => StackedDrgWinning64GiBV1,
            fil_RegisteredPoStProof::StackedDrgWindow2KiBV1 => StackedDrgWindow2KiBV1,
            fil_RegisteredPoStProof::StackedDrgWindow8MiBV1 => StackedDrgWindow8MiBV1,
            fil_RegisteredPoStProof::StackedDrgWindow512MiBV1 => StackedDrgWindow512MiBV1,
            fil_RegisteredPoStProof::StackedDrgWindow32GiBV1 => StackedDrgWindow32GiBV1,
            fil_RegisteredPoStProof::StackedDrgWindow64GiBV1 => StackedDrgWindow64GiBV1,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum fil_RegisteredAggregationProof {
    SnarkPackV1,
}

impl From<RegisteredAggregationProof> for fil_RegisteredAggregationProof {
    fn from(other: RegisteredAggregationProof) -> Self {
        match other {
            RegisteredAggregationProof::SnarkPackV1 => fil_RegisteredAggregationProof::SnarkPackV1,
        }
    }
}

impl From<fil_RegisteredAggregationProof> for RegisteredAggregationProof {
    fn from(other: fil_RegisteredAggregationProof) -> Self {
        match other {
            fil_RegisteredAggregationProof::SnarkPackV1 => RegisteredAggregationProof::SnarkPackV1,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum fil_RegisteredUpdateProof {
    StackedDrg2KiBV1,
    StackedDrg8MiBV1,
    StackedDrg512MiBV1,
    StackedDrg32GiBV1,
    StackedDrg64GiBV1,
}

impl From<RegisteredUpdateProof> for fil_RegisteredUpdateProof {
    fn from(other: RegisteredUpdateProof) -> Self {
        match other {
            RegisteredUpdateProof::StackedDrg2KiBV1 => fil_RegisteredUpdateProof::StackedDrg2KiBV1,
            RegisteredUpdateProof::StackedDrg8MiBV1 => fil_RegisteredUpdateProof::StackedDrg8MiBV1,
            RegisteredUpdateProof::StackedDrg512MiBV1 => {
                fil_RegisteredUpdateProof::StackedDrg512MiBV1
            }
            RegisteredUpdateProof::StackedDrg32GiBV1 => {
                fil_RegisteredUpdateProof::StackedDrg32GiBV1
            }
            RegisteredUpdateProof::StackedDrg64GiBV1 => {
                fil_RegisteredUpdateProof::StackedDrg64GiBV1
            }
        }
    }
}

impl From<fil_RegisteredUpdateProof> for RegisteredUpdateProof {
    fn from(other: fil_RegisteredUpdateProof) -> Self {
        match other {
            fil_RegisteredUpdateProof::StackedDrg2KiBV1 => RegisteredUpdateProof::StackedDrg2KiBV1,
            fil_RegisteredUpdateProof::StackedDrg8MiBV1 => RegisteredUpdateProof::StackedDrg8MiBV1,
            fil_RegisteredUpdateProof::StackedDrg512MiBV1 => {
                RegisteredUpdateProof::StackedDrg512MiBV1
            }
            fil_RegisteredUpdateProof::StackedDrg32GiBV1 => {
                RegisteredUpdateProof::StackedDrg32GiBV1
            }
            fil_RegisteredUpdateProof::StackedDrg64GiBV1 => {
                RegisteredUpdateProof::StackedDrg64GiBV1
            }
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct fil_PublicPieceInfo {
    pub num_bytes: u64,
    pub comm_p: [u8; 32],
}

impl From<fil_PublicPieceInfo> for PieceInfo {
    fn from(x: fil_PublicPieceInfo) -> Self {
        let fil_PublicPieceInfo { num_bytes, comm_p } = x;
        PieceInfo {
            commitment: comm_p,
            size: UnpaddedBytesAmount(num_bytes),
        }
    }
}

#[allow(non_camel_case_types)]
pub type fil_VanillaProof = fil_Bytes;

#[allow(non_camel_case_types)]
pub type fil_AggregateProof = fil_Result<fil_VanillaProof>;

#[derive(Clone, Debug)]
pub struct PoStProof {
    pub registered_proof: RegisteredPoStProof,
    pub proof: Vec<u8>,
}

#[repr(C)]
#[derive(Clone)]
pub struct fil_PoStProof {
    pub registered_proof: fil_RegisteredPoStProof,
    pub proof: fil_Bytes,
}

impl From<fil_PoStProof> for PoStProof {
    fn from(other: fil_PoStProof) -> Self {
        PoStProof {
            registered_proof: other.registered_proof.into(),
            proof: other.proof.to_vec(),
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct PartitionSnarkProof {
    pub registered_proof: RegisteredPoStProof,
    pub proof: Vec<u8>,
}

#[repr(C)]
#[derive(Clone)]
pub struct fil_PartitionSnarkProof {
    pub registered_proof: fil_RegisteredPoStProof,
    pub proof: fil_Bytes,
}

impl From<fil_PartitionSnarkProof> for PartitionSnarkProof {
    fn from(other: fil_PartitionSnarkProof) -> Self {
        PartitionSnarkProof {
            registered_proof: other.registered_proof.into(),
            proof: other.proof.to_vec(),
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct PartitionProof {
    pub proof: Vec<u8>,
}

#[allow(non_camel_case_types)]
pub type fil_PartitionProof = fil_Bytes;

#[repr(C)]
#[derive(Clone)]
pub struct fil_PrivateReplicaInfo {
    pub registered_proof: fil_RegisteredPoStProof,
    pub cache_dir_path: fil_Bytes,
    pub comm_r: [u8; 32],
    pub replica_path: fil_Bytes,
    pub sector_id: u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct fil_PublicReplicaInfo {
    pub registered_proof: fil_RegisteredPoStProof,
    pub comm_r: [u8; 32],
    pub sector_id: u64,
}

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_GenerateWinningPoStSectorChallenge {
    pub error_msg: *mut libc::c_char,
    pub status_code: FCPResponseStatus,
    pub ids_ptr: *mut u64,
    pub ids_len: libc::size_t,
}

impl Default for fil_GenerateWinningPoStSectorChallenge {
    fn default() -> fil_GenerateWinningPoStSectorChallenge {
        fil_GenerateWinningPoStSectorChallenge {
            ids_len: 0,
            ids_ptr: ptr::null_mut(),
            error_msg: ptr::null_mut(),
            status_code: FCPResponseStatus::FCPNoError,
        }
    }
}

code_and_message_impl!(fil_GenerateWinningPoStSectorChallenge);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_GenerateFallbackSectorChallengesResponse {
    pub error_msg: *mut libc::c_char,
    pub status_code: FCPResponseStatus,
    pub ids_ptr: *mut u64,
    pub ids_len: libc::size_t,
    pub challenges_ptr: *mut u64,
    pub challenges_len: libc::size_t,
    pub challenges_stride: libc::size_t,
}

impl Default for fil_GenerateFallbackSectorChallengesResponse {
    fn default() -> fil_GenerateFallbackSectorChallengesResponse {
        fil_GenerateFallbackSectorChallengesResponse {
            challenges_len: 0,
            challenges_stride: 0,
            challenges_ptr: ptr::null_mut(),
            ids_len: 0,
            ids_ptr: ptr::null_mut(),
            error_msg: ptr::null_mut(),
            status_code: FCPResponseStatus::FCPNoError,
        }
    }
}

code_and_message_impl!(fil_GenerateFallbackSectorChallengesResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_GenerateSingleVanillaProofResponse {
    pub error_msg: *mut libc::c_char,
    pub vanilla_proof: fil_VanillaProof,
    pub status_code: FCPResponseStatus,
}

impl Default for fil_GenerateSingleVanillaProofResponse {
    fn default() -> fil_GenerateSingleVanillaProofResponse {
        fil_GenerateSingleVanillaProofResponse {
            error_msg: ptr::null_mut(),
            vanilla_proof: Default::default(),
            status_code: FCPResponseStatus::FCPNoError,
        }
    }
}

code_and_message_impl!(fil_GenerateSingleVanillaProofResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_GenerateWinningPoStResponse {
    pub error_msg: *mut libc::c_char,
    pub proofs_len: libc::size_t,
    pub proofs_ptr: *mut fil_PoStProof,
    pub status_code: FCPResponseStatus,
}

impl Default for fil_GenerateWinningPoStResponse {
    fn default() -> fil_GenerateWinningPoStResponse {
        fil_GenerateWinningPoStResponse {
            error_msg: ptr::null_mut(),
            proofs_len: 0,
            proofs_ptr: ptr::null_mut(),
            status_code: FCPResponseStatus::FCPNoError,
        }
    }
}

code_and_message_impl!(fil_GenerateWinningPoStResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_GenerateWindowPoStResponse {
    pub error_msg: *mut libc::c_char,
    pub proofs_len: libc::size_t,
    pub proofs_ptr: *mut fil_PoStProof,
    pub faulty_sectors_len: libc::size_t,
    pub faulty_sectors_ptr: *mut u64,
    pub status_code: FCPResponseStatus,
}

impl Default for fil_GenerateWindowPoStResponse {
    fn default() -> fil_GenerateWindowPoStResponse {
        fil_GenerateWindowPoStResponse {
            error_msg: ptr::null_mut(),
            proofs_len: 0,
            proofs_ptr: ptr::null_mut(),
            faulty_sectors_len: 0,
            faulty_sectors_ptr: ptr::null_mut(),
            status_code: FCPResponseStatus::FCPNoError,
        }
    }
}

code_and_message_impl!(fil_GenerateWindowPoStResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_GenerateSingleWindowPoStWithVanillaResponse {
    pub error_msg: *mut libc::c_char,
    pub partition_proof: fil_PartitionSnarkProof,
    pub faulty_sectors_len: libc::size_t,
    pub faulty_sectors_ptr: *mut u64,
    pub status_code: FCPResponseStatus,
}

impl Default for fil_GenerateSingleWindowPoStWithVanillaResponse {
    fn default() -> fil_GenerateSingleWindowPoStWithVanillaResponse {
        fil_GenerateSingleWindowPoStWithVanillaResponse {
            error_msg: ptr::null_mut(),
            partition_proof: fil_PartitionSnarkProof {
                registered_proof: fil_RegisteredPoStProof::StackedDrgWinning2KiBV1,
                proof: Default::default(),
            },
            faulty_sectors_len: 0,
            faulty_sectors_ptr: ptr::null_mut(),
            status_code: FCPResponseStatus::FCPNoError,
        }
    }
}

code_and_message_impl!(fil_GenerateSingleWindowPoStWithVanillaResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_GetNumPartitionForFallbackPoStResponse {
    pub error_msg: *mut libc::c_char,
    pub status_code: FCPResponseStatus,
    pub num_partition: libc::size_t,
}

impl Default for fil_GetNumPartitionForFallbackPoStResponse {
    fn default() -> fil_GetNumPartitionForFallbackPoStResponse {
        fil_GetNumPartitionForFallbackPoStResponse {
            error_msg: ptr::null_mut(),
            num_partition: 0,
            status_code: FCPResponseStatus::FCPNoError,
        }
    }
}

code_and_message_impl!(fil_GetNumPartitionForFallbackPoStResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_MergeWindowPoStPartitionProofsResponse {
    pub error_msg: *mut libc::c_char,
    pub proof: fil_PoStProof,
    pub status_code: FCPResponseStatus,
}

impl Default for fil_MergeWindowPoStPartitionProofsResponse {
    fn default() -> fil_MergeWindowPoStPartitionProofsResponse {
        fil_MergeWindowPoStPartitionProofsResponse {
            error_msg: ptr::null_mut(),
            proof: fil_PoStProof {
                registered_proof: fil_RegisteredPoStProof::StackedDrgWinning2KiBV1,
                proof: Default::default(),
            },
            status_code: FCPResponseStatus::FCPNoError,
        }
    }
}

code_and_message_impl!(fil_MergeWindowPoStPartitionProofsResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_WriteWithAlignmentResponse {
    pub comm_p: [u8; 32],
    pub error_msg: *mut libc::c_char,
    pub left_alignment_unpadded: u64,
    pub status_code: FCPResponseStatus,
    pub total_write_unpadded: u64,
}

impl Default for fil_WriteWithAlignmentResponse {
    fn default() -> fil_WriteWithAlignmentResponse {
        fil_WriteWithAlignmentResponse {
            comm_p: Default::default(),
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            left_alignment_unpadded: 0,
            total_write_unpadded: 0,
        }
    }
}

code_and_message_impl!(fil_WriteWithAlignmentResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_WriteWithoutAlignmentResponse {
    pub comm_p: [u8; 32],
    pub error_msg: *mut libc::c_char,
    pub status_code: FCPResponseStatus,
    pub total_write_unpadded: u64,
}

impl Default for fil_WriteWithoutAlignmentResponse {
    fn default() -> fil_WriteWithoutAlignmentResponse {
        fil_WriteWithoutAlignmentResponse {
            comm_p: Default::default(),
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            total_write_unpadded: 0,
        }
    }
}

code_and_message_impl!(fil_WriteWithoutAlignmentResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_SealPreCommitPhase1Response {
    pub error_msg: *mut libc::c_char,
    pub status_code: FCPResponseStatus,
    pub seal_pre_commit_phase1_output_ptr: *mut u8,
    pub seal_pre_commit_phase1_output_len: libc::size_t,
}

impl Default for fil_SealPreCommitPhase1Response {
    fn default() -> fil_SealPreCommitPhase1Response {
        fil_SealPreCommitPhase1Response {
            error_msg: ptr::null_mut(),
            status_code: FCPResponseStatus::FCPNoError,
            seal_pre_commit_phase1_output_ptr: ptr::null_mut(),
            seal_pre_commit_phase1_output_len: 0,
        }
    }
}

code_and_message_impl!(fil_SealPreCommitPhase1Response);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_FauxRepResponse {
    pub error_msg: *mut libc::c_char,
    pub status_code: FCPResponseStatus,
    pub commitment: [u8; 32],
}

impl Default for fil_FauxRepResponse {
    fn default() -> fil_FauxRepResponse {
        fil_FauxRepResponse {
            error_msg: ptr::null_mut(),
            status_code: FCPResponseStatus::FCPNoError,
            commitment: Default::default(),
        }
    }
}

code_and_message_impl!(fil_FauxRepResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_SealPreCommitPhase2Response {
    pub error_msg: *mut libc::c_char,
    pub status_code: FCPResponseStatus,
    pub registered_proof: fil_RegisteredSealProof,
    pub comm_d: [u8; 32],
    pub comm_r: [u8; 32],
}

impl Default for fil_SealPreCommitPhase2Response {
    fn default() -> fil_SealPreCommitPhase2Response {
        fil_SealPreCommitPhase2Response {
            error_msg: ptr::null_mut(),
            status_code: FCPResponseStatus::FCPNoError,
            registered_proof: fil_RegisteredSealProof::StackedDrg2KiBV1,
            comm_d: Default::default(),
            comm_r: Default::default(),
        }
    }
}

code_and_message_impl!(fil_SealPreCommitPhase2Response);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_SealCommitPhase1Response {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub seal_commit_phase1_output_ptr: *mut u8,
    pub seal_commit_phase1_output_len: libc::size_t,
}

impl Default for fil_SealCommitPhase1Response {
    fn default() -> fil_SealCommitPhase1Response {
        fil_SealCommitPhase1Response {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            seal_commit_phase1_output_ptr: ptr::null_mut(),
            seal_commit_phase1_output_len: 0,
        }
    }
}

code_and_message_impl!(fil_SealCommitPhase1Response);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_SealCommitPhase2Response {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub proof_ptr: *mut u8,
    pub proof_len: libc::size_t,
    pub commit_inputs_ptr: *mut fil_AggregationInputs,
    pub commit_inputs_len: libc::size_t,
}

impl From<&fil_SealCommitPhase2Response> for SealCommitPhase2Output {
    fn from(other: &fil_SealCommitPhase2Response) -> Self {
        let proof = unsafe { clone_as_vec_from_parts(other.proof_ptr, other.proof_len) };

        SealCommitPhase2Output { proof }
    }
}

impl Default for fil_SealCommitPhase2Response {
    fn default() -> fil_SealCommitPhase2Response {
        fil_SealCommitPhase2Response {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            proof_ptr: ptr::null_mut(),
            proof_len: 0,
            commit_inputs_ptr: ptr::null_mut(),
            commit_inputs_len: 0,
        }
    }
}

impl Clone for fil_SealCommitPhase2Response {
    fn clone(&self) -> Self {
        let proof_ptr = unsafe { clone_box_parts(self.proof_ptr, self.proof_len) };
        let commit_inputs_ptr =
            unsafe { clone_box_parts(self.commit_inputs_ptr, self.commit_inputs_len) };

        fil_SealCommitPhase2Response {
            status_code: self.status_code,
            error_msg: self.error_msg,
            proof_ptr,
            proof_len: self.proof_len,
            commit_inputs_ptr,
            commit_inputs_len: self.commit_inputs_len,
        }
    }
}

code_and_message_impl!(fil_SealCommitPhase2Response);

#[repr(C)]
#[derive(Clone, DropStructMacro)]
pub struct fil_AggregationInputs {
    pub comm_r: fil_32ByteArray,
    pub comm_d: fil_32ByteArray,
    pub sector_id: u64,
    pub ticket: fil_32ByteArray,
    pub seed: fil_32ByteArray,
}

impl Default for fil_AggregationInputs {
    fn default() -> fil_AggregationInputs {
        fil_AggregationInputs {
            comm_r: fil_32ByteArray::default(),
            comm_d: fil_32ByteArray::default(),
            sector_id: 0,
            ticket: fil_32ByteArray::default(),
            seed: fil_32ByteArray::default(),
        }
    }
}

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_UnsealRangeResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
}

impl Default for fil_UnsealRangeResponse {
    fn default() -> fil_UnsealRangeResponse {
        fil_UnsealRangeResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
        }
    }
}

code_and_message_impl!(fil_UnsealRangeResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_VerifySealResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub is_valid: bool,
}

impl Default for fil_VerifySealResponse {
    fn default() -> fil_VerifySealResponse {
        fil_VerifySealResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            is_valid: false,
        }
    }
}

code_and_message_impl!(fil_VerifySealResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_VerifyAggregateSealProofResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub is_valid: bool,
}

impl Default for fil_VerifyAggregateSealProofResponse {
    fn default() -> fil_VerifyAggregateSealProofResponse {
        fil_VerifyAggregateSealProofResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            is_valid: false,
        }
    }
}

code_and_message_impl!(fil_VerifyAggregateSealProofResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_VerifyWinningPoStResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub is_valid: bool,
}

impl Default for fil_VerifyWinningPoStResponse {
    fn default() -> fil_VerifyWinningPoStResponse {
        fil_VerifyWinningPoStResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            is_valid: false,
        }
    }
}

code_and_message_impl!(fil_VerifyWinningPoStResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_VerifyWindowPoStResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub is_valid: bool,
}

impl Default for fil_VerifyWindowPoStResponse {
    fn default() -> fil_VerifyWindowPoStResponse {
        fil_VerifyWindowPoStResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            is_valid: false,
        }
    }
}

code_and_message_impl!(fil_VerifyWindowPoStResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_FinalizeTicketResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub ticket: [u8; 32],
}

impl Default for fil_FinalizeTicketResponse {
    fn default() -> Self {
        fil_FinalizeTicketResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            ticket: [0u8; 32],
        }
    }
}

code_and_message_impl!(fil_FinalizeTicketResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_GeneratePieceCommitmentResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub comm_p: [u8; 32],
    /// The number of unpadded bytes in the original piece plus any (unpadded)
    /// alignment bytes added to create a whole merkle tree.
    pub num_bytes_aligned: u64,
}

impl Default for fil_GeneratePieceCommitmentResponse {
    fn default() -> fil_GeneratePieceCommitmentResponse {
        fil_GeneratePieceCommitmentResponse {
            status_code: FCPResponseStatus::FCPNoError,
            comm_p: Default::default(),
            error_msg: ptr::null_mut(),
            num_bytes_aligned: 0,
        }
    }
}

code_and_message_impl!(fil_GeneratePieceCommitmentResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_GenerateDataCommitmentResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub comm_d: [u8; 32],
}

impl Default for fil_GenerateDataCommitmentResponse {
    fn default() -> fil_GenerateDataCommitmentResponse {
        fil_GenerateDataCommitmentResponse {
            status_code: FCPResponseStatus::FCPNoError,
            comm_d: Default::default(),
            error_msg: ptr::null_mut(),
        }
    }
}

code_and_message_impl!(fil_GenerateDataCommitmentResponse);

///

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_StringResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub string_val: *mut libc::c_char,
}

impl Default for fil_StringResponse {
    fn default() -> fil_StringResponse {
        fil_StringResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            string_val: ptr::null_mut(),
        }
    }
}

code_and_message_impl!(fil_StringResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_ClearCacheResponse {
    pub error_msg: *mut libc::c_char,
    pub status_code: FCPResponseStatus,
}

impl Default for fil_ClearCacheResponse {
    fn default() -> fil_ClearCacheResponse {
        fil_ClearCacheResponse {
            error_msg: ptr::null_mut(),
            status_code: FCPResponseStatus::FCPNoError,
        }
    }
}

code_and_message_impl!(fil_ClearCacheResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_EmptySectorUpdateEncodeIntoResponse {
    pub error_msg: *mut libc::c_char,
    pub status_code: FCPResponseStatus,
    pub comm_r_new: [u8; 32],
    pub comm_r_last_new: [u8; 32],
    pub comm_d_new: [u8; 32],
}

impl Default for fil_EmptySectorUpdateEncodeIntoResponse {
    fn default() -> fil_EmptySectorUpdateEncodeIntoResponse {
        fil_EmptySectorUpdateEncodeIntoResponse {
            error_msg: ptr::null_mut(),
            status_code: FCPResponseStatus::FCPNoError,
            comm_r_new: Default::default(),
            comm_r_last_new: Default::default(),
            comm_d_new: Default::default(),
        }
    }
}

code_and_message_impl!(fil_EmptySectorUpdateEncodeIntoResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_EmptySectorUpdateDecodeFromResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
}

impl Default for fil_EmptySectorUpdateDecodeFromResponse {
    fn default() -> fil_EmptySectorUpdateDecodeFromResponse {
        fil_EmptySectorUpdateDecodeFromResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
        }
    }
}

code_and_message_impl!(fil_EmptySectorUpdateDecodeFromResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_EmptySectorUpdateRemoveEncodedDataResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
}

impl Default for fil_EmptySectorUpdateRemoveEncodedDataResponse {
    fn default() -> fil_EmptySectorUpdateRemoveEncodedDataResponse {
        fil_EmptySectorUpdateRemoveEncodedDataResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
        }
    }
}

code_and_message_impl!(fil_EmptySectorUpdateRemoveEncodedDataResponse);

#[repr(C)]
pub struct fil_EmptySectorUpdateProofResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub proof_len: libc::size_t,
    pub proof_ptr: *mut u8,
}

impl Default for fil_EmptySectorUpdateProofResponse {
    fn default() -> fil_EmptySectorUpdateProofResponse {
        fil_EmptySectorUpdateProofResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            proof_len: 0,
            proof_ptr: ptr::null_mut(),
        }
    }
}

impl Drop for fil_EmptySectorUpdateProofResponse {
    fn drop(&mut self) {
        unsafe {
            drop_box_from_parts(self.proof_ptr);
            free_c_str(self.error_msg as *mut libc::c_char);
        }
    }
}

code_and_message_impl!(fil_EmptySectorUpdateProofResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_PartitionProofResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub proofs_len: libc::size_t,
    pub proofs_ptr: *mut fil_PartitionProof,
}

impl Default for fil_PartitionProofResponse {
    fn default() -> fil_PartitionProofResponse {
        fil_PartitionProofResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            proofs_len: 0,
            proofs_ptr: ptr::null_mut(),
        }
    }
}

code_and_message_impl!(fil_PartitionProofResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_VerifyPartitionProofResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub is_valid: bool,
}

impl Default for fil_VerifyPartitionProofResponse {
    fn default() -> fil_VerifyPartitionProofResponse {
        fil_VerifyPartitionProofResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            is_valid: false,
        }
    }
}

code_and_message_impl!(fil_VerifyPartitionProofResponse);

#[repr(C)]
#[derive(DropStructMacro)]
pub struct fil_VerifyEmptySectorUpdateProofResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *mut libc::c_char,
    pub is_valid: bool,
}

impl Default for fil_VerifyEmptySectorUpdateProofResponse {
    fn default() -> fil_VerifyEmptySectorUpdateProofResponse {
        fil_VerifyEmptySectorUpdateProofResponse {
            status_code: FCPResponseStatus::FCPNoError,
            error_msg: ptr::null_mut(),
            is_valid: false,
        }
    }
}

code_and_message_impl!(fil_VerifyEmptySectorUpdateProofResponse);
