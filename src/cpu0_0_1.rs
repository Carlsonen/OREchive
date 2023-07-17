use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Cpus0_0_1 {
    pub format_version: String,
    pub last_modified: String,
    pub cpus: Vec<Cpu0_0_1>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cpu0_0_1 {
    pub name: String,
    pub builders: Vec<String>,
    pub date_finished: String,
    pub significant_programs: Vec<String>,
    pub schematic_name: Option<String>,
    pub isa_link: Option<String>,
    pub specifications: Specifications0_0_1,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Specifications0_0_1 {
    pub clock_period: u32,
    pub gpio_count: u32,
    pub custom_assembler: bool,
    pub mchprs_compatible: bool,
    pub urcl_compatible: bool,
    pub architecture: Architecture0_0_1,
    pub alu: Alu0_0_1,
    pub memory: Memory0_0_1,
    pub control: Control0_0_1,
    pub programming: Programming0_0_1,
    pub hazards: Hazards0_0_1,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Architecture0_0_1 {
    pub architecture_type: String,
    pub pipelined: bool,
    pub pipeline_type: Option<String>,
    pub superscalar: bool,
    pub instant_mechanics: bool,
    pub instant_architecture: bool,
    pub has_pistons: bool,
    pub data_numeric_base: u32,
    pub word_size_bits: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Alu0_0_1 {
    pub full_bitwise_support: bool,
    pub right_shifting: bool,
    pub multi_shifting: bool,
    pub rotating: bool,
    pub hardware_multiplication: bool,
    pub hardware_division: bool,
    pub floating_point_support: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Memory0_0_1 {
    pub general_purpose_registers: u32,
    pub ram_bytes: u32,
    pub ram_cache: u32,
    pub indirect_access: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Control0_0_1 {
    pub absolute_branching: bool,
    pub indirect_branching: bool,
    pub branch_prediction: bool,
    pub predication_support: bool,
    pub interrupt_support: bool,
    pub callstack_capabilities: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Programming0_0_1 {
    pub instruction_operands: u32,
    pub instruction_width: u32,
    pub variable_instruction_length: bool,
    pub total_program_memory: u32,
    pub effective_max_instructions: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Hazards0_0_1 {
    pub alu_raw: bool,
    pub memory_raw: bool,
    pub branch: bool,
}
