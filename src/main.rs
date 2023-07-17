mod cpu0_0_1;
use cpu0_0_1::*;

fn main() {
    let brummig = Cpu0_0_1 {
        name: "Brummig".to_string(),
        builders: vec!["oscar91".to_string()],
        date_finished: "2022-06-30".to_string(),
        significant_programs: vec!["Maze explorer".to_string(), "Prime sieve".to_string()],
        schematic_name: None,
        isa_link: None,
        specifications: Specifications0_0_1 {
            clock_period: 6,
            gpio_count: 4,
            custom_assembler: true,
            mchprs_compatible: true,
            urcl_compatible: false,
            architecture: Architecture0_0_1 {
                architecture_type: "Harvard".to_string(),
                pipelined: true,
                pipeline_type: Some("Waterfall".to_string()),
                superscalar: false,
                instant_mechanics: false,
                instant_architecture: false,
                has_pistons: false,
                data_numeric_base: 2,
                word_size_bits: 8,
            },
            alu: Alu0_0_1 {
                full_bitwise_support: true,
                right_shifting: true,
                multi_shifting: false,
                rotating: false,
                hardware_multiplication: false,
                hardware_division: false,
                floating_point_support: false,
            },
            memory: Memory0_0_1 {
                general_purpose_registers: 9,
                ram_bytes: 64,
                ram_cache: 0,
                indirect_access: true,
            },
            control: Control0_0_1 {
                absolute_branching: true,
                indirect_branching: false,
                branch_prediction: false,
                predication_support: false,
                interrupt_support: false,
                callstack_capabilities: true,
            },
            programming: Programming0_0_1 {
                instruction_operands: 3,
                instruction_width: 16,
                variable_instruction_length: false,
                total_program_memory: 512,
                effective_max_instructions: 256,
            },
            hazards: Hazards0_0_1 {
                alu_raw: false,
                memory_raw: true,
                branch: true,
            },
        },
    };
    let cpus = Cpus0_0_1 {
        format_version: "0.0.1".to_string(),
        last_modified: "2023-07-17".to_string(),
        cpus: vec![brummig],
    };
    let toml_txt = toml::to_string(&cpus).unwrap();
    std::fs::write("cpus.toml", toml_txt).unwrap();
}
