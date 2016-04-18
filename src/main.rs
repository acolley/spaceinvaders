#![feature(slice_patterns)]

extern crate clap;
#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::Read;

use clap::{Arg, App};

#[derive(Debug)]
struct Addr(u8, u8);

/// An 8080 Register
#[derive(Debug)]
enum Reg {
    A, B, C, D, E, H, L
}

#[derive(Debug)]
enum RegPair {
    BC,
    DE,
    HL,
    SP
}

/// An 8080 Op code and its data
#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Op {
    // MOV -> (dst, src)
    MOV_RR(Reg, Reg),
    MOV_MR(Reg),
    MOV_RM(Reg),

    // MVI -> (dst, data)
    MVI_RD(Reg, u8),
    MVI_M(u8),
    MVI_A(u8),

    // LXI -> (dst, data)
    LXI(RegPair, u8, u8),

    // LDA -> src
    LDA(Addr),

    // STA -> dst
    STA(Addr),

    // LHLD -> src
    LHLD(Addr),

    // SHLD -> dst
    SHLD(Addr),

    // LDAX -> src
    LDAX(RegPair),

    // STAX -> dst
    STAX(RegPair),

    XCHG,

    ADD_R(Reg),
    ADD_M,

    ADI(u8),

    ADC_R(Reg),
    ADC_M,

    ACI(u8),

    SUB_R(Reg),
    SUB_M,

    SUI(u8),

    SBB_R(Reg),
    SBB_M,

    SBI(u8),

    INR_R(Reg),
    INR_M,

    DCR_R(Reg),
    DCR_M,

    INX(RegPair),

    DCX(RegPair),

    DAD(RegPair),

    DAA,

    ANA_R(Reg),
    ANA_M,

    ANI(u8),

    XRA_R(Reg),
    XRA_M,

    XRI(u8),

    ORA_R(Reg),
    ORA_M,

    ORI(u8),

    CMP_R(Reg),
    CMP_M,

    CPI(u8),

    RLC,
    RRC,
    RAL,
    RAR,
    CMA,
    CMC,
    STC,

    RNC,
    JNC(Addr),
    JC(Addr),
    CNC(Addr),
    CC(Addr),
    RC,
    RPO,
    JPO(Addr),
    CPO(Addr),

    RPE,
    JPE(Addr),
    CPE(Addr),
    RP,
    JP(Addr),
    CP(Addr),

    JMP(Addr),
    CALL(Addr),
    RET,
    RST(u8),
    PCHL,

    RNZ,
    RZ,
    JNZ(Addr),
    JZ(Addr),
    CNZ(Addr),
    CZ(Addr),

    PUSH_SP(RegPair),
    PUSH_PSW,

    POP_SP(RegPair),
    POP_PSW,

    XTHL,
    SPHL,

    IN(u8),
    OUT(u8),

    EI,
    DI,
    HTL,

    NOP,

    SIM,
    RIM,
    RM,
    JM(Addr),
    CM(Addr),
}

// TOOD: change data[x] to data.get_unchecked for performance
fn bytes_to_lxi_b(data: &[u8]) -> Op {
    Op::LXI(RegPair::BC, data[1], data[0])
}

fn bytes_to_lxi_d(data: &[u8]) -> Op {
    Op::LXI(RegPair::DE, data[1], data[0])
}

fn bytes_to_lxi_h(data: &[u8]) -> Op {
    Op::LXI(RegPair::HL, data[1], data[0])
}

fn bytes_to_lxi_sp(data: &[u8]) -> Op {
    Op::LXI(RegPair::SP, data[1], data[0])
}

fn bytes_to_mvi_b(data: &[u8]) -> Op {
    Op::MVI_RD(Reg::B, data[0])
}

fn bytes_to_mvi_c(data: &[u8]) -> Op {
    Op::MVI_RD(Reg::C, data[0])
}

fn bytes_to_mvi_d(data: &[u8]) -> Op {
    Op::MVI_RD(Reg::D, data[0])
}

fn bytes_to_mvi_e(data: &[u8]) -> Op {
    Op::MVI_RD(Reg::E, data[0])
}

fn bytes_to_mvi_h(data: &[u8]) -> Op {
    Op::MVI_RD(Reg::H, data[0])
}

fn bytes_to_mvi_l(data: &[u8]) -> Op {
    Op::MVI_RD(Reg::L, data[0])
}

fn bytes_to_mvi_m(data: &[u8]) -> Op {
    Op::MVI_M(data[0])
}

fn bytes_to_mvi_a(data: &[u8]) -> Op {
    Op::MVI_A(data[0])
}

fn bytes_to_shld(data: &[u8]) -> Op {
    Op::SHLD(Addr(data[0], data[1]))
}

fn bytes_to_lhld(data: &[u8]) -> Op {
    Op::LHLD(Addr(data[0], data[1]))
}

fn bytes_to_sta(data: &[u8]) -> Op {
    Op::STA(Addr(data[0], data[1]))
}

fn bytes_to_lda(data: &[u8]) -> Op {
    Op::LDA(Addr(data[0], data[1]))
}

fn bytes_to_jnz(data: &[u8]) -> Op {
    Op::JNZ(Addr(data[0], data[1]))
}

fn bytes_to_jz(data: &[u8]) -> Op {
    Op::JZ(Addr(data[0], data[1]))
}

fn bytes_to_jnc(data: &[u8]) -> Op {
    Op::JNC(Addr(data[0], data[1]))
}

fn bytes_to_jc(data: &[u8]) -> Op {
    Op::JC(Addr(data[0], data[1]))
}

fn bytes_to_jmp(data: &[u8]) -> Op {
    Op::JMP(Addr(data[0], data[1]))
}

fn bytes_to_cnz(data: &[u8]) -> Op {
    Op::CNZ(Addr(data[0], data[1]))
}

fn bytes_to_cnc(data: &[u8]) -> Op {
    Op::CNC(Addr(data[0], data[1]))
}

fn bytes_to_cc(data: &[u8]) -> Op {
    Op::CC(Addr(data[0], data[1]))
}

fn bytes_to_cz(data: &[u8]) -> Op {
    Op::CZ(Addr(data[0], data[1]))
}

fn bytes_to_cp(data: &[u8]) -> Op {
    Op::CP(Addr(data[0], data[1]))
}

fn bytes_to_adi(data: &[u8]) -> Op {
    Op::ADI(data[0])
}

fn bytes_to_aci(data: &[u8]) -> Op {
    Op::ACI(data[0])
}

fn bytes_to_ani(data: &[u8]) -> Op {
    Op::ANI(data[0])
}

fn bytes_to_call(data: &[u8]) -> Op {
    Op::CALL(Addr(data[0], data[1]))
}

fn bytes_to_out(data: &[u8]) -> Op {
    Op::OUT(data[0])
}

fn bytes_to_in(data: &[u8]) -> Op {
    Op::IN(data[0])
}

fn bytes_to_sui(data: &[u8]) -> Op {
    Op::SUI(data[0])
}

fn bytes_to_sbi(data: &[u8]) -> Op {
    Op::SBI(data[0])
}

fn bytes_to_jpo(data: &[u8]) -> Op {
    Op::JPO(Addr(data[0], data[1]))
}

fn bytes_to_jpe(data: &[u8]) -> Op {
    Op::JPE(Addr(data[0], data[1]))
}

fn bytes_to_jp(data: &[u8]) -> Op {
    Op::JP(Addr(data[0], data[1]))
}

fn bytes_to_jm(data: &[u8]) -> Op {
    Op::JM(Addr(data[0], data[1]))
}

fn bytes_to_cpo(data: &[u8]) -> Op {
    Op::CPO(Addr(data[0], data[1]))
}

fn bytes_to_cpe(data: &[u8]) -> Op {
    Op::CPE(Addr(data[0], data[1]))
}

fn bytes_to_cm(data: &[u8]) -> Op {
    Op::CM(Addr(data[0], data[1]))
}

fn bytes_to_xri(data: &[u8]) -> Op {
    Op::XRI(data[0])
}

fn bytes_to_ori(data: &[u8]) -> Op {
    Op::ORI(data[0])
}

fn bytes_to_cpi(data: &[u8]) -> Op {
    Op::CPI(data[0])
}

// named!(disassemble_lxi <Op>, tap!(data: take!(2) => value!(Op::LXI(RegPair::BC, data[0], data[1]))));

named!(disassemble(&[u8]) -> Vec<Op>,
    many1!(
        switch!(take!(1),
            [0x00] => value!(Op::NOP) |
            [0x01] => map!(take!(2), bytes_to_lxi_b) |
            [0x02] => value!(Op::STAX(RegPair::BC)) |
            [0x03] => value!(Op::INX(RegPair::BC)) |
            [0x04] => value!(Op::INR_R(Reg::B)) |
            [0x05] => value!(Op::DCR_R(Reg::B)) |
            [0x06] => map!(take!(1), bytes_to_mvi_b) |
            [0x07] => value!(Op::RLC) |
            // [0x08] => 
            [0x09] => value!(Op::DAD(RegPair::BC)) |
            [0x0a] => value!(Op::LDAX(RegPair::BC)) |
            [0x0b] => value!(Op::DCX(RegPair::BC)) |
            [0x0c] => value!(Op::INR_R(Reg::C)) |
            [0x0d] => value!(Op::DCR_R(Reg::C)) |
            [0x0e] => map!(take!(1), bytes_to_mvi_c) |
            [0x0f] => value!(Op::RRC) |
            // [0x10] =>
            [0x11] => map!(take!(2), bytes_to_lxi_d) |
            [0x12] => value!(Op::STAX(RegPair::DE)) |
            [0x13] => value!(Op::INX(RegPair::DE)) |
            [0x14] => value!(Op::INR_R(Reg::D)) |
            [0x15] => value!(Op::DCR_R(Reg::D)) |
            [0x16] => map!(take!(1), bytes_to_mvi_d) |
            [0x17] => value!(Op::RAL) |
            // [0x18] => 
            [0x19] => value!(Op::DAD(RegPair::DE)) |
            [0x1a] => value!(Op::LDAX(RegPair::DE)) |
            [0x1b] => value!(Op::DCX(RegPair::DE)) |
            [0x1c] => value!(Op::INR_R(Reg::E)) |
            [0x1d] => value!(Op::DCR_R(Reg::E)) |
            [0x1e] => map!(take!(1), bytes_to_mvi_e) |
            [0x1f] => value!(Op::RAR) |
            [0x20] => value!(Op::RIM) |
            [0x21] => map!(take!(2), bytes_to_lxi_h) |
            [0x22] => map!(take!(2), bytes_to_shld) |
            [0x23] => value!(Op::INX(RegPair::HL)) |
            [0x24] => value!(Op::INR_R(Reg::H)) |
            [0x25] => value!(Op::DCR_R(Reg::H)) |
            [0x26] => map!(take!(1), bytes_to_mvi_h) |
            [0x27] => value!(Op::DAA) |
            // [0x28] => 
            [0x29] => value!(Op::DAD(RegPair::HL)) |
            [0x2a] => map!(take!(2), bytes_to_lhld) |
            [0x2b] => value!(Op::DCX(RegPair::HL)) |
            [0x2c] => value!(Op::INR_R(Reg::L)) |
            [0x2d] => value!(Op::DCR_R(Reg::L)) |
            [0x2e] => map!(take!(1), bytes_to_mvi_l) |
            [0x2f] => value!(Op::CMA) |
            [0x30] => value!(Op::SIM) |
            [0x31] => map!(take!(2), bytes_to_lxi_sp) |
            [0x32] => map!(take!(2), bytes_to_sta) |
            [0x33] => value!(Op::INX(RegPair::SP)) |
            [0x34] => value!(Op::INR_M) |
            [0x35] => value!(Op::DCR_M) |
            [0x36] => map!(take!(1), bytes_to_mvi_m) |
            [0x37] => value!(Op::STC) |
            // [0x38] => 
            [0x39] => value!(Op::DAD(RegPair::SP)) |
            [0x3a] => map!(take!(2), bytes_to_lda) |
            [0x3b] => value!(Op::DCX(RegPair::SP)) |
            [0x3c] => value!(Op::INR_R(Reg::A)) |
            [0x3d] => value!(Op::DCR_R(Reg::A)) |
            [0x3e] => map!(take!(1), bytes_to_mvi_a) |
            [0x3f] => value!(Op::CMC) |
            [0x40] => value!(Op::MOV_RR(Reg::B, Reg::B)) |
            [0x41] => value!(Op::MOV_RR(Reg::B, Reg::C)) |
            [0x42] => value!(Op::MOV_RR(Reg::B, Reg::D)) |
            [0x43] => value!(Op::MOV_RR(Reg::B, Reg::E)) |
            [0x44] => value!(Op::MOV_RR(Reg::B, Reg::H)) |
            [0x45] => value!(Op::MOV_RR(Reg::B, Reg::L)) |
            [0x46] => value!(Op::MOV_RM(Reg::B)) |
            [0x47] => value!(Op::MOV_RR(Reg::B, Reg::A)) |
            [0x48] => value!(Op::MOV_RR(Reg::C, Reg::B)) |
            [0x49] => value!(Op::MOV_RR(Reg::C, Reg::C)) |
            [0x4a] => value!(Op::MOV_RR(Reg::C, Reg::D)) |
            [0x4b] => value!(Op::MOV_RR(Reg::C, Reg::E)) |
            [0x4c] => value!(Op::MOV_RR(Reg::C, Reg::H)) |
            [0x4d] => value!(Op::MOV_RR(Reg::C, Reg::L)) |
            [0x4e] => value!(Op::MOV_RM(Reg::C)) |
            [0x4f] => value!(Op::MOV_RR(Reg::C, Reg::A)) |
            [0x50] => value!(Op::MOV_RR(Reg::D, Reg::B)) |
            [0x51] => value!(Op::MOV_RR(Reg::D, Reg::C)) |
            [0x52] => value!(Op::MOV_RR(Reg::D, Reg::D)) |
            [0x53] => value!(Op::MOV_RR(Reg::D, Reg::E)) |
            [0x54] => value!(Op::MOV_RR(Reg::D, Reg::H)) |
            [0x55] => value!(Op::MOV_RR(Reg::D, Reg::L)) |
            [0x56] => value!(Op::MOV_RM(Reg::D)) |
            [0x57] => value!(Op::MOV_RR(Reg::D, Reg::A)) |
            [0x58] => value!(Op::MOV_RR(Reg::E, Reg::B)) |
            [0x59] => value!(Op::MOV_RR(Reg::E, Reg::C)) |
            [0x5a] => value!(Op::MOV_RR(Reg::E, Reg::D)) |
            [0x5b] => value!(Op::MOV_RR(Reg::E, Reg::E)) |
            [0x5c] => value!(Op::MOV_RR(Reg::E, Reg::H)) |
            [0x5d] => value!(Op::MOV_RR(Reg::E, Reg::L)) |
            [0x5e] => value!(Op::MOV_RM(Reg::E)) |
            [0x5f] => value!(Op::MOV_RR(Reg::E, Reg::A)) |
            [0x60] => value!(Op::MOV_RR(Reg::H, Reg::B)) |
            [0x61] => value!(Op::MOV_RR(Reg::H, Reg::C)) |
            [0x62] => value!(Op::MOV_RR(Reg::H, Reg::D)) |
            [0x63] => value!(Op::MOV_RR(Reg::H, Reg::E)) |
            [0x64] => value!(Op::MOV_RR(Reg::H, Reg::H)) |
            [0x65] => value!(Op::MOV_RR(Reg::H, Reg::L)) |
            [0x66] => value!(Op::MOV_RM(Reg::H)) |
            [0x67] => value!(Op::MOV_RR(Reg::H, Reg::A)) |
            [0x68] => value!(Op::MOV_RR(Reg::L, Reg::B)) |
            [0x69] => value!(Op::MOV_RR(Reg::L, Reg::C)) |
            [0x6a] => value!(Op::MOV_RR(Reg::L, Reg::D)) |
            [0x6b] => value!(Op::MOV_RR(Reg::L, Reg::E)) |
            [0x6c] => value!(Op::MOV_RR(Reg::L, Reg::H)) |
            [0x6d] => value!(Op::MOV_RR(Reg::L, Reg::L)) |
            [0x6e] => value!(Op::MOV_RM(Reg::L)) |
            [0x6f] => value!(Op::MOV_RR(Reg::L, Reg::A)) |
            [0x70] => value!(Op::MOV_MR(Reg::B)) |
            [0x71] => value!(Op::MOV_MR(Reg::C)) |
            [0x72] => value!(Op::MOV_MR(Reg::D)) |
            [0x73] => value!(Op::MOV_MR(Reg::E)) |
            [0x74] => value!(Op::MOV_MR(Reg::H)) |
            [0x75] => value!(Op::MOV_MR(Reg::L)) |
            [0x76] => value!(Op::HTL) |
            [0x77] => value!(Op::MOV_MR(Reg::A)) |
            [0x78] => value!(Op::MOV_RR(Reg::A, Reg::B)) |
            [0x79] => value!(Op::MOV_RR(Reg::A, Reg::C)) |
            [0x7a] => value!(Op::MOV_RR(Reg::A, Reg::D)) |
            [0x7b] => value!(Op::MOV_RR(Reg::A, Reg::E)) |
            [0x7c] => value!(Op::MOV_RR(Reg::A, Reg::H)) |
            [0x7d] => value!(Op::MOV_RR(Reg::A, Reg::L)) |
            [0x7e] => value!(Op::MOV_RM(Reg::A)) |
            [0x7f] => value!(Op::MOV_RR(Reg::A, Reg::A)) |
            [0x80] => value!(Op::ADD_R(Reg::B)) |
            [0x81] => value!(Op::ADD_R(Reg::C)) |
            [0x82] => value!(Op::ADD_R(Reg::D)) |
            [0x83] => value!(Op::ADD_R(Reg::E)) |
            [0x84] => value!(Op::ADD_R(Reg::H)) |
            [0x85] => value!(Op::ADD_R(Reg::L)) |
            [0x86] => value!(Op::ADD_M) |
            [0x87] => value!(Op::ADD_R(Reg::A)) |
            // ...
            [0x90] => value!(Op::SUB_R(Reg::B)) |
            [0x91] => value!(Op::SUB_R(Reg::C)) |
            [0x92] => value!(Op::SUB_R(Reg::D)) |
            [0x93] => value!(Op::SUB_R(Reg::E)) |
            [0x94] => value!(Op::SUB_R(Reg::H)) |
            [0x95] => value!(Op::SUB_R(Reg::L)) |
            [0x96] => value!(Op::SUB_M) |
            [0x97] => value!(Op::SUB_R(Reg::A)) |
            // ...
            [0xa0] => value!(Op::ANA_R(Reg::B)) |
            [0xa1] => value!(Op::ANA_R(Reg::C)) |
            [0xa2] => value!(Op::ANA_R(Reg::D)) |
            [0xa3] => value!(Op::ANA_R(Reg::E)) |
            [0xa4] => value!(Op::ANA_R(Reg::H)) |
            [0xa5] => value!(Op::ANA_R(Reg::L)) |
            [0xa6] => value!(Op::ANA_M) |
            [0xa7] => value!(Op::ANA_R(Reg::A)) |
            [0xa8] => value!(Op::XRA_R(Reg::B)) |
            [0xa9] => value!(Op::XRA_R(Reg::C)) |
            [0xaa] => value!(Op::XRA_R(Reg::D)) |
            [0xab] => value!(Op::XRA_R(Reg::E)) |
            [0xac] => value!(Op::XRA_R(Reg::H)) |
            [0xad] => value!(Op::XRA_R(Reg::L)) |
            [0xae] => value!(Op::XRA_M) |
            [0xaf] => value!(Op::XRA_R(Reg::A)) |
            [0xb0] => value!(Op::ORA_R(Reg::B)) |
            [0xb1] => value!(Op::ORA_R(Reg::C)) |
            [0xb2] => value!(Op::ORA_R(Reg::D)) |
            [0xb3] => value!(Op::ORA_R(Reg::E)) |
            [0xb4] => value!(Op::ORA_R(Reg::H)) |
            [0xb5] => value!(Op::ORA_R(Reg::L)) |
            [0xb6] => value!(Op::ORA_M) |
            [0xb7] => value!(Op::ORA_R(Reg::A)) |
            [0xb8] => value!(Op::CMP_R(Reg::B)) |
            [0xb9] => value!(Op::CMP_R(Reg::C)) |
            [0xba] => value!(Op::CMP_R(Reg::D)) |
            [0xbb] => value!(Op::CMP_R(Reg::E)) |
            [0xbc] => value!(Op::CMP_R(Reg::H)) |
            [0xbd] => value!(Op::CMP_R(Reg::L)) |
            [0xbe] => value!(Op::CMP_M) |
            [0xbf] => value!(Op::CMP_R(Reg::A)) |
            [0xc0] => value!(Op::RNZ) |
            [0xc1] => value!(Op::POP_SP(RegPair::BC)) |
            [0xc2] => map!(take!(2), bytes_to_jnz) |
            [0xc3] => map!(take!(2), bytes_to_jmp) |
            [0xc4] => map!(take!(2), bytes_to_cnz) |
            [0xc5] => value!(Op::PUSH_SP(RegPair::BC)) |
            [0xc6] => map!(take!(1), bytes_to_adi) |
            [0xc7] => value!(Op::RST(0)) |
            [0xc8] => value!(Op::RZ) |
            [0xc9] => value!(Op::RET) |
            [0xca] => map!(take!(2), bytes_to_jz) |
            // [0xcb] => 
            [0xcc] => map!(take!(2), bytes_to_cz) |
            [0xcd] => map!(take!(2), bytes_to_call) |
            [0xce] => map!(take!(1), bytes_to_aci) |
            [0xcf] => value!(Op::RST(1)) |
            [0xd0] => value!(Op::RNC) |
            [0xd1] => value!(Op::POP_SP(RegPair::DE)) |
            [0xd2] => map!(take!(2), bytes_to_jnc) |
            [0xd3] => map!(take!(1), bytes_to_out) |
            [0xd4] => map!(take!(2), bytes_to_cnc) |
            [0xd5] => value!(Op::PUSH_SP(RegPair::DE)) |
            [0xd6] => map!(take!(1), bytes_to_sui) |
            [0xd7] => value!(Op::RST(2)) |
            [0xd8] => value!(Op::RC) |
            // [0xd9] => 
            [0xda] => map!(take!(2), bytes_to_jc) |
            [0xdb] => map!(take!(1), bytes_to_in) |
            [0xdc] => map!(take!(2), bytes_to_cc) |
            // [0xdd] => 
            [0xde] => map!(take!(1), bytes_to_sbi) |
            [0xdf] => value!(Op::RST(3)) |
            [0xe0] => value!(Op::RPO) |
            [0xe1] => value!(Op::POP_SP(RegPair::HL)) |
            [0xe2] => map!(take!(2), bytes_to_jpo) |
            [0xe3] => value!(Op::XTHL) |
            [0xe4] => map!(take!(2), bytes_to_cpo) |
            [0xe5] => value!(Op::PUSH_SP(RegPair::HL)) |
            [0xe6] => map!(take!(1), bytes_to_ani) |
            [0xe7] => value!(Op::RST(4)) |
            [0xe8] => value!(Op::RPE) |
            [0xe9] => value!(Op::PCHL) |
            [0xea] => map!(take!(2), bytes_to_jpe) |
            [0xeb] => value!(Op::XCHG) |
            [0xec] => map!(take!(2), bytes_to_cpe) |
            // [0xed] =>
            [0xee] => map!(take!(1), bytes_to_xri) |
            [0xef] => value!(Op::RST(5)) |
            [0xf0] => value!(Op::RP) |
            [0xf1] => value!(Op::POP_PSW) |
            [0xf2] => map!(take!(2), bytes_to_jp) |
            [0xf3] => value!(Op::DI) |
            [0xf4] => map!(take!(2), bytes_to_cp) |
            [0xf5] => value!(Op::PUSH_PSW) |
            [0xf6] => map!(take!(1), bytes_to_ori) |
            [0xf7] => value!(Op::RST(6)) |
            [0xf8] => value!(Op::RM) |
            [0xf9] => value!(Op::SPHL) |
            [0xfa] => map!(take!(2), bytes_to_jm) |
            [0xfb] => value!(Op::EI) |
            [0xfc] => map!(take!(2), bytes_to_cm) |
            // [0xfd] => 
            [0xfe] => map!(take!(1), bytes_to_cpi) |
            [0xff] => value!(Op::RST(7))
        )
    )
);

struct Options {
    filename: String,
}

fn get_opts() -> Options {
    let matches = App::new("emu8080")
                       .version("0.1")
                       .arg(Arg::with_name("FILENAME")
                            .required(true))
                       .get_matches();
    let filename = matches.value_of("FILENAME").unwrap();
    Options {
        filename: String::from(filename),
    }
}

fn main() {
    let options = get_opts();
    let mut file = File::open(options.filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    println!("{:?}", disassemble(&buf));
}